use crate::agent::Agent;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::SinkExt;
use futures_util::StreamExt;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;
use tokio::task::{JoinError, JoinHandle};
use tokio::try_join;
use tokio_tungstenite::tungstenite::Error;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use super::handlers::handle_game_ended::handle_game_ended;
use super::handlers::handle_next_move::handle_next_move;
use super::handlers::handle_prepare_to_game::handle_prepare_to_game;
use super::packet::packet::Packet;

pub struct WebSocketClient {
    read_task: JoinHandle<()>,
    writer_task: JoinHandle<()>,
}

impl WebSocketClient {
    pub async fn connect(
        host: &str,
        port: u16,
        code: &str,
        nickname: &str,
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

        // Create channel for sending messages to the WebSocket
        let (tx, rx) = tokio::sync::mpsc::channel(100);

        // Create WebSocket writer task
        let writer_task = Self::create_writer_task(write, rx);

        // Create agent
        let agent = Arc::new(Mutex::new(None));

        // Create WebSocket reader task
        let read_task = Self::create_reader_task(read, tx, agent);

        Ok(WebSocketClient {
            read_task,
            writer_task,
        })
    }

    pub async fn run(self) -> Result<((), ()), JoinError> {
        try_join!(self.read_task, self.writer_task)
    }

    pub fn construct_url(host: &str, port: u16, code: &str, nickname: &str) -> String {
        let mut url = format!("ws://{}:{}/?nickname={}", host, port, nickname);

        url.push_str("&typeOfPacketType=string");
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
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                match write.send(message).await {
                    Ok(_) => {}
                    Err(e) => eprintln!("[System] 🌋 WebSocket send error -> {}", e),
                }
            }
        })
    }

    fn create_reader_task(
        mut read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
        tx: Sender<Message>,
        agent: Arc<Mutex<Option<Agent>>>,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            while let Some(message) = read.next().await {
                match message {
                    Ok(message) => {
                        tokio::spawn(Self::process_message(message, tx.clone(), agent.clone()));
                    }
                    Err(e) => {
                        eprintln!("[System] 🌋 WebSocket receive error -> {}", e);
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
                if let Err(e) = Self::process_text_message(message.clone(), tx, agent).await {
                    eprintln!("[System] 🚨 Error processing text message -> {}", e);
                    eprintln!("[System] 🚨 Text Message -> {}", message);
                }
            }
            Message::Ping(message) => {
                println!("[System] 🏓 Received Ping");
                tx.send(Message::Pong(message)).await.unwrap();
            }
            Message::Pong(_) => {
                println!("[System] 🏓 Received Pong");
            }
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

            Packet::ConnectionAccepted => {
                println!("[System] 🎉 Connection accepted");
            }
            Packet::ConnectionRejected { reason } => {
                println!("[System] 🚨 Connection rejected -> {}", reason);
            }

            Packet::LobbyData(lobby_data) => {
                println!("[System] 🎳 Lobby data received");
                handle_prepare_to_game(tx, agent, lobby_data).await?
            }

            Packet::LobbyDeleted => {
                println!("[System] 🚪 Lobby deleted");
            }

            Packet::GameStart => println!("[System] 🎲 Game started"),
            Packet::GameState(raw_game_state) => {
                // println!("🎮 Game state received");
                handle_next_move(tx, agent, raw_game_state).await?
            }

            Packet::GameEnd(game_end) => {
                println!("[System] 🏁 Game ended");
                handle_game_ended(agent, game_end).await?
            }

            // Warnings
            Packet::PlayerAlreadyMadeActionWarning => {
                println!("[System] 🚨 Player already made action warning");
            }
            Packet::MissingGameStateIdWarning => {
                println!("[System] 🚨 Missing game state id warning");
            }
            Packet::SlowResponseWarning => {
                println!("[System] 🚨 Slow response warning");
            }
            Packet::ActionIgnoredDueToDeadWarning => {
                println!("[System] 🚨 Action ignored due to dead warning");
            }

            Packet::CustomWarning { message } => {
                println!("[System] 🚨 Warning -> {}", message);
            }

            // Errors
            Packet::InvalidPacketTypeError => {
                println!("[System] 🚨 Client sent an invalid packet type error");
            }
            Packet::InvalidPacketUsageError => {
                println!("[System] 🚨 Client used packet in invalid way");
            }

            // These packets are never send by the server
            Packet::Pong
            | Packet::Movement { .. }
            | Packet::Rotation { .. }
            | Packet::AbilityUse { .. }
            | Packet::Pass { .. } => {
                unreachable!()
            }
        };

        Ok(())
    }
}
