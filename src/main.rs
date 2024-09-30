use clap::Parser;
use hackathon_2024_h2_rust_client::{args::Args, ws_client::ws_client::WebSocketClient};

#[tokio::main]
async fn main() {
    let Args {
        host,
        port,
        code,
        nickname,
    } = Args::parse();

    println!("[System] 🚀 Starting client...");
    let websocket_client = match WebSocketClient::connect(&host, port, &code, &nickname).await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("[System] 🌋 Error connecting to the server -> {}", e);
            return;
        }
    };

    if let Err(e) = websocket_client.run().await {
        eprintln!("[System] 🌋 Error running WebSocket client -> {}", e);
    }
}
