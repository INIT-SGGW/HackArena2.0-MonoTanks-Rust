use crate::agent::MyAgent;
use crate::game::agent_trait::Agent;
use crate::ws_client::packet::dto::lobby_data::LobbyData;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{timeout, Duration};
use tokio_tungstenite::tungstenite::Message;

pub async fn handle_prepare_to_game(
    _tx: tokio::sync::mpsc::Sender<Message>,
    agent: Arc<Mutex<Option<MyAgent>>>,
    lobby_data: LobbyData,
) -> Result<(), String> {
    // Set the timeout duration
    // TODO: Make this configurable
    let timeout_duration = Duration::from_secs(5);

    // Spawn a blocking task to create the new MyAgent and acquire the lock
    let agent_creation = tokio::task::spawn_blocking(move || {
        let mut agent_guard = agent.blocking_lock();
        *agent_guard = Some(MyAgent::new(lobby_data));
    });

    // Wrap the blocking task with a timeout
    match timeout(timeout_duration, agent_creation).await {
        Ok(Ok(_)) => (),
        Ok(Err(e)) => return Err(format!("Task failed: {}", e)),
        Err(_) => return Err("Agent creation timed out".to_string()),
    }

    Ok(())
}
