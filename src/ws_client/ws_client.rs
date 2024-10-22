use crate::agent::Agent;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tokio_tungstenite::tungstenite::Error;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use tokio_util::sync::CancellationToken;

use super::handlers::handle_game_ended::handle_game_ended;
use super::handlers::handle_next_move::handle_next_move;
use super::handlers::handle_on_warning_received::handle_on_warning_received;
use super::handlers::handle_prepare_to_game::handle_prepare_to_game;
use super::packet::packet::Packet;
use super::packet::warning::Warning;

pub struct WebSocketClient {
    read_task: JoinHandle<Result<(), Error>>,
    writer_task: JoinHandle<Result<(), Error>>,
    cancel_token: CancellationToken,
}

impl WebSocketClient {
    pub async fn connect(
        host: &str,
        port: u16,
        code: &str,
        nickname: &str,
        cancel_token: CancellationToken,
    ) -> Result<WebSocketClient, Error> {
        // Construct proper url
        let url = Self::construct_url(host, port, code, nickname);

        // Connect to the server
        println!("[System] 📞 Connecting to the server: {}", url);
        let websocket_stream = match connect_async(&url).await {
            Ok((stream, _)) => {
                println!("[System] 🌟 Successfully connected to the server");
                stream
            }
            Err(e) => return Err(e),
        };

        // Split the stream into write and read parts
        let (write, read) = websocket_stream.split();

        let (tx, rx) = tokio::sync::mpsc::channel(100);
        let agent = Arc::new(Mutex::new(None));

        let writer_task = Self::create_writer_task(write, rx, cancel_token.clone());
        let read_task = Self::create_reader_task(read, tx, agent, cancel_token.clone());

        Ok(WebSocketClient {
            read_task,
            writer_task,
            cancel_token,
        })
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let WebSocketClient {
            read_task,
            writer_task,
            cancel_token,
        } = self;

        tokio::select! {
            _ = cancel_token.cancelled() => {
                println!("[System] 👋 Client shutting down...");
            }
            read_result = read_task => {
                if let Err(e) = read_result? {
                    eprintln!("[System] 📚 Read task error: {}", e);
                }
            }
            write_result = writer_task => {
                if let Err(e) = write_result? {
                    eprintln!("[System] 📝 Write task error: {}", e);
                }
            }
        }

        Ok(())
    }

    pub fn construct_url(host: &str, port: u16, code: &str, nickname: &str) -> String {
        let mut url = format!("ws://{}:{}/?nickname={}", host, port, nickname);

        url.push_str("&enumSerializationFormat=string");
        url.push_str("&playerType=hackathonBot");

        if !code.is_empty() {
            url.push_str("&joinCode=");
            url.push_str(code);
        }

        url
    }

    fn create_writer_task(
        mut write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
        mut rx: Receiver<Message>,
        cancel_token: CancellationToken,
    ) -> JoinHandle<Result<(), Error>> {
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    message = rx.recv() => {
                        match message {
                            Some(message) => {
                                if let Err(e) = write.send(message).await {
                                    eprintln!("[System] 🌋 WebSocket send error: {}", e);
                                    break Err(e);
                                }
                            }
                            None => break Ok(()),
                        }
                    }
                    _ = cancel_token.cancelled() => {
                        if let Err(e) = write.close().await {
                            eprintln!("[System] 🌋 Error closing WebSocket connection: {}", e);
                        }
                        break Ok(());
                    }
                }
            }
        })
    }

    fn create_reader_task(
        mut read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
        tx: Sender<Message>,
        agent: Arc<Mutex<Option<Agent>>>,
        cancel_token: CancellationToken,
    ) -> JoinHandle<Result<(), Error>> {
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    message = read.next() => {
                        match message {
                            Some(Ok(message)) => {
                                Self::process_message(message, tx.clone(), agent.clone()).await;
                            }
                            Some(Err(e)) => {
                                eprintln!("[System] 🌋 WebSocket receive error: {}", e);
                                cancel_token.cancel();
                                break Err(e);
                            }
                            None => {
                                println!("[System] 🔌 Connection closed by server");
                                cancel_token.cancel();
                                break Ok(());
                            }
                        }
                    }
                    _ = cancel_token.cancelled() => {
                        break Ok(());
                    }
                }
            }
        })
    }

    async fn process_message(
        message: Message,
        tx: Sender<Message>,
        agent: Arc<Mutex<Option<Agent>>>,
    ) {
        match message {
            Message::Text(message) => {
                let tx_clone = tx.clone();
                let agent_clone = agent.clone();
                tokio::task::spawn(async move {
                    if let Err(e) =
                        Self::process_text_message(message.clone(), tx_clone, agent_clone).await
                    {
                        eprintln!("[System] 🚨 Error processing text message -> {}", e);
                    }
                });
            }
            Message::Ping(message) => {
                tx.send(Message::Pong(message)).await.unwrap();
            }
            Message::Pong(_) => {}
            Message::Close(_) => {
                println!("[System] 🚪 Connection closed");
            }
            Message::Binary(_) => {
                println!("[System] 🔢 Received Binary message");
            }
            Message::Frame(_) => {
                println!("[System] 🖼 Received Frame message");
            }
        }
    }

    async fn process_text_message(
        message: String,
        tx: tokio::sync::mpsc::Sender<Message>,
        agent: Arc<Mutex<Option<Agent>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let packet: Packet = serde_json::from_str(&message)
            .map_err(|e| format!("🚨 Error parsing message -> {}", e))?;

        match packet {
            Packet::Ping => tx
                .send(Message::Text(Packet::Pong.into()))
                .await
                .map_err(|e| format!("🚨 Error sending Pong -> {}", e))?,

            Packet::ConnectionRejected { reason } => {
                println!("[System] 🚨 Connection rejected -> {}", reason);
            }

            Packet::ConnectionAccepted => {
                println!("[System] 🎉 Connection accepted");

                match tx
                    .send(Message::Text(Packet::LobbyDataRequest.into()))
                    .await
                {
                    Ok(_) => println!("[System] 🎳 Lobby data request sent"),
                    Err(e) => eprintln!("[System] 🚨 Error sending LobbyDataRequest -> {}", e),
                }
            }

            Packet::GameNotStarted => {
                println!("[System] 🕒 Game not started yet");
            }

            Packet::GameInProgress => {
                println!("[System] 🏃 Game in progress");
            }

            Packet::GameStarting => {
                println!("[System] 🎲 Game starting");

                // Wait until agent is not None
                while agent.lock().await.is_none() {
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                }

                tx.send(Message::Text(Packet::ReadyToReceiveGameState.into()))
                    .await
                    .map_err(|e| format!("🚨 Error sending ReadyToReceiveGameState -> {}", e))?;
            }

            Packet::LobbyData(lobby_data) => {
                println!("[System] 🎳 Lobby data received");
                handle_prepare_to_game(tx, agent, lobby_data).await?
            }

            Packet::GameStarted => println!("[System] 🎲 Game started"),
            Packet::GameState(raw_game_state) => {
                // println!("🎮 Game state received");
                handle_next_move(tx, agent, raw_game_state).await?
            }

            Packet::GameEnded(game_end) => {
                println!("[System] 🏁 Game ended");
                handle_game_ended(agent, game_end).await?
            }

            // Warnings
            Packet::PlayerAlreadyMadeActionWarning => {
                let warning = Warning::PlayerAlreadyMadeActionWarning;
                handle_on_warning_received(agent, warning).await?;
            }
            Packet::MissingGameStateIdWarning => {
                let warning = Warning::MissingGameStateIdWarning;
                handle_on_warning_received(agent, warning).await?;
            }
            Packet::SlowResponseWarning => {
                let warning = Warning::SlowResponseWarning;
                handle_on_warning_received(agent, warning).await?;
            }
            Packet::ActionIgnoredDueToDeadWarning => {
                let warning = Warning::ActionIgnoredDueToDeadWarning;
                handle_on_warning_received(agent, warning).await?;
            }
            Packet::CustomWarning { message } => {
                let warning = Warning::CustomWarning { message };
                handle_on_warning_received(agent, warning).await?;
            }

            // Errors
            Packet::InvalidPacketTypeError => {
                println!("[System] 🚨 Client sent an invalid packet type error");
            }
            Packet::InvalidPacketUsageError => {
                println!("[System] 🚨 Client used packet in invalid way");
            }
            Packet::InvalidPayloadError { message } => {
                println!("[System] 🚨 Invalid payload error -> {}", message);
            }

            // These packets are never send by the server
            Packet::Pong
            | Packet::LobbyDataRequest
            | Packet::GameStatusRequest
            | Packet::ReadyToReceiveGameState { .. }
            | Packet::Movement { .. }
            | Packet::Rotation { .. }
            | Packet::AbilityUse { .. }
            | Packet::Pass { .. } => {
                println!("[System] ⚠️ Unexpected packet received: {:?}", packet);
            }
        };

        Ok(())
    }
}
