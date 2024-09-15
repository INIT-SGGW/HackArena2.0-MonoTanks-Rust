use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

pub fn create_writer_task(
    mut write: SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
) -> (
    tokio::task::JoinHandle<()>,
    tokio::sync::mpsc::Sender<Message>,
) {
    // Create channel for sending messages to the WebSocket

    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    // Spawn WebSocket writer task
    let writer_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            match write.send(message).await {
                Ok(_) => {}
                Err(e) => eprintln!("WebSocket send error: {}", e),
            }
        }
    });

    (writer_task, tx)
}
