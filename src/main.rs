use clap::Parser;
use hackathon_2024_h2_rust_client::{args::Args, ws_client::ws_client::WebSocketClient};
use tokio::signal;
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let Args {
        host,
        port,
        code,
        nickname,
    } = Args::parse();

    println!("[System] 🚀 Starting client...");

    // Create a single cancellation token for both CTRL+C and connection loss
    let cancel_token = CancellationToken::new();

    // Spawn a task to handle CTRL+C
    let ctrl_c_handler = tokio::spawn(handle_ctrl_c(cancel_token.clone()));

    let websocket_client =
        match WebSocketClient::connect(&host, port, &code, &nickname, cancel_token.clone()).await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("[System] 🌋 Error connecting to the server -> {}", e);
                return;
            }
        };

    // Run the WebSocket client
    let client_result = websocket_client.run().await;

    // Cancel the CTRL+C handler and wait for it to complete
    cancel_token.cancel();
    let _ = ctrl_c_handler.await;

    match client_result {
        Ok(_) => {}
        Err(e) => eprintln!("[System] 🌋 Error running WebSocket client: {}", e),
    }
}

async fn handle_ctrl_c(cancel_token: CancellationToken) {
    tokio::select! {
        _ = signal::ctrl_c() => {
            println!("[System] 🛑 Received CTRL+C, initiating shutdown...");
            cancel_token.cancel();
        }
        _ = cancel_token.cancelled() => {}
    }
}
