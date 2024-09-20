use clap::Parser;
use hackaton_2024_h2_rust_client::{args::Args, ws_client::ws_client::WebSocketClient};

#[tokio::main]
async fn main() {
    let Args {
        host,
        port,
        code,
        nickname,
        debug_quick_join,
    } = Args::parse();

    println!("🚀 Starting client...");
    let websocket_client =
        match WebSocketClient::connect(&host, port, &code, &nickname, debug_quick_join).await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("🌋 Error connecting to the server -> {}", e);
                return;
            }
        };

    if let Err(e) = websocket_client.run().await {
        eprintln!("🌋 Error running WebSocket client -> {}", e);
    }
}
