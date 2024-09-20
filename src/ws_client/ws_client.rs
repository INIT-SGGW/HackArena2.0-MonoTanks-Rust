use crate::agent::MyAgent;
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use std::sync::Arc;
use tokio::{
    net::TcpStream,
    sync::{
        mpsc::{Receiver, Sender},
        Mutex,
    },
    task::{JoinError, JoinHandle},
    try_join,
};
use tokio_tungstenite::tungstenite::Error;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use super::{
    handlers::{
        handle_game_ended::handle_game_ended, handle_next_move::handle_next_move,
        handle_prepare_to_game::handle_prepare_to_game,
    },
    packet::{packet::Packet, packet_type_enum::PacketType},
};

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
        debug_quick_join: bool,
    ) -> Result<WebSocketClient, Error> {
        // Construct proper url
        let url = Self::construct_url(host, port, code, nickname, debug_quick_join);

        // Connect to the server
        println!("📞 Connecting to the server: {}", url);
        let websocket_stream = match connect_async(&url).await {
            Ok((stream, _)) => {
                println!("🎉 Successfully connected to the server");
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

    pub fn construct_url(
        host: &str,
        port: u16,
        code: &str,
        nickname: &str,
        debug_quick_join: bool,
    ) -> String {
        let mut url = format!("ws://{}:{}/?nickname={}", host, port, nickname);

        url.push_str("&typeOfPacketType=string");

        if debug_quick_join {
            url.push_str("&quickJoin=true");
        }

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
                    Err(e) => eprintln!("🌋 WebSocket send error -> {}", e),
                }
            }
        })
    }

    fn create_reader_task(
        mut read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>,
        tx: Sender<Message>,
        agent: Arc<Mutex<Option<MyAgent>>>,
    ) -> JoinHandle<()> {
        tokio::spawn(async move {
            while let Some(message) = read.next().await {
                match message {
                    Ok(message) => {
                        tokio::spawn(Self::process_message(message, tx.clone(), agent.clone()));
                    }
                    Err(e) => {
                        eprintln!("🌋 WebSocket receive error -> {}", e);
                    }
                }
            }
        })
    }

    async fn process_message(
        message: Message,
        tx: Sender<Message>,
        agent: Arc<Mutex<Option<MyAgent>>>,
    ) {
        match message {
            Message::Text(message) => {
                if let Err(e) = Self::process_text_message(message.clone(), tx, agent).await {
                    eprintln!("🚨 Error processing text message -> {}", e);
                    eprintln!("🚨 Text Message -> {}", message);
                }
            }
            Message::Ping(message) => {
                println!("🏓 Received Ping");
                tx.send(Message::Pong(message)).await.unwrap();
            }
            Message::Pong(_) => {
                println!("🏓 Received Pong");
            }
            Message::Close(_) => {
                println!("🚪 Connection closed");
            }
            Message::Binary(_) => {
                println!("🔢 Received Binary message");
            }
            Message::Frame(_) => {
                println!("🖼 Received Frame message");
            }
        }
    }

    async fn process_text_message(
        message: String,
        tx: tokio::sync::mpsc::Sender<Message>,
        agent: Arc<Mutex<Option<MyAgent>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // println!("📩 Received message: {}", message);

        let packet: Packet = serde_json::from_str(&message)
            .map_err(|e| format!("🚨 Error parsing JSON -> {}", e))?;

        match packet.packet_type {
            PacketType::LobbyData => {
                handle_prepare_to_game(tx, agent, packet.payload).await?;
            }

            PacketType::GameStart => {}

            PacketType::GameState => {
                handle_next_move(tx, agent, packet.payload).await?;
            }

            PacketType::GameEnded => {
                handle_game_ended(agent, packet.payload).await?;
            }

            PacketType::Ping => {
                Self::respond_to_ping(tx).await?;
            }

            PacketType::TankMovement
            | PacketType::TankRotation
            | PacketType::TankShoot
            | PacketType::Ready => {
                unreachable!()
            }

            PacketType::Pong | PacketType::Unknown => {
                unimplemented!();
            }
        };

        Ok(())
    }

    async fn respond_to_ping(tx: tokio::sync::mpsc::Sender<Message>) -> Result<(), String> {
        let response = Packet::construct_pong_packet();
        let response_string = serde_json::to_string(&response)
            .map_err(|e| format!("🚨 Error serializing Pong -> {}", e))?;
        tx.send(Message::Text(response_string))
            .await
            .map_err(|e| format!("🚨 Error sending Pong -> {}", e))?;

        Ok(())
    }
}
