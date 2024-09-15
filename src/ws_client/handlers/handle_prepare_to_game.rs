use crate::agent::MyAgent;
use crate::game::{agent_trait::Agent, game_info::GameInfo};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{timeout, Duration};
use tokio_tungstenite::tungstenite::Message;

pub async fn handle_prepare_to_game(
    _tx: tokio::sync::mpsc::Sender<Message>,
    agent: Arc<Mutex<Option<MyAgent>>>,
    payload: serde_json::Value,
) -> Result<(), String> {
    // Parse the JSON with serde_json
    let game_info_result: Result<GameInfo, String> = payload.try_into();

    let game_info = match game_info_result {
        Ok(game_info) => game_info,
        Err(e) => return Err(format!("Failed to parse game info: {}", e)),
    };

    // Set the timeout duration
    let timeout_duration = Duration::from_secs(5);

    // Spawn a blocking task to create the new MyAgent and acquire the lock
    let agent_creation = tokio::task::spawn_blocking(move || {
        let mut agent_guard = agent.blocking_lock();
        *agent_guard = Some(MyAgent::new(game_info));
    });

    // Wrap the blocking task with a timeout
    match timeout(timeout_duration, agent_creation).await {
        Ok(Ok(_)) => (),
        Ok(Err(e)) => return Err(format!("Task failed: {}", e)),
        Err(_) => return Err("Agent creation timed out".to_string()),
    }

    // let response = serde_json::json!({
    //     "type": PacketType::Ready as u64,
    //     "payload": {}
    // }).to_string();

    // tx.send(Message::Text(response)).await.unwrap();

    Ok(())
}
