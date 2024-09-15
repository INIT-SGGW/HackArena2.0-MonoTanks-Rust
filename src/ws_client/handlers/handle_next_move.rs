use crate::agent::MyAgent;
use crate::game::agent_trait::Agent;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio::time::{timeout, Duration};
use tokio_tungstenite::tungstenite::Message;

pub async fn handle_next_move(
    tx: Sender<Message>,
    agent: Arc<Mutex<Option<MyAgent>>>,
    payload: Value,
) -> Result<(), String> {
    // Set the timeout duration
    let timeout_duration = Duration::from_secs(5);

    // Spawn the blocking task with a timeout
    let next_move_task = tokio::task::spawn_blocking(move || -> Result<String, String> {
        let game_state = payload
            .try_into()
            .map_err(|e| format!("Failed to parse into game state, {}", e))?;

        let agent_response = {
            let mut agent_lock = agent
                .try_lock()
                .map_err(|_| "Failed to lock agent, it is already in use")?;

            match agent_lock.as_mut() {
                Some(agent) => Ok(agent.next_move(game_state)),
                None => Err("Agent not initialized".to_string()),
            }
        }
        .map_err(|e| format!("Failed to get agent response, {}", e))?;

        let response_to_server =
            serde_json::to_string(&agent_response).map_err(|e| e.to_string())?;

        Ok(response_to_server)
    });

    // Apply the timeout
    let response_json = match timeout(timeout_duration, next_move_task).await {
        Ok(Ok(result)) => result?,
        Ok(Err(e)) => return Err(format!("Task failed: {}", e)),
        Err(_) => return Err("Task timed out".to_string()),
    };

    // Send the response
    tx.send(Message::Text(response_json))
        .await
        .map_err(|_| "Failed to send message")?;

    Ok(())
}
