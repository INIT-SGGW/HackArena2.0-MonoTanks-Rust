use crate::agent::MyAgent;
use crate::game::agent_trait::Agent;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{timeout, Duration};

pub async fn handle_game_ended(
    agent: Arc<Mutex<Option<MyAgent>>>,
    payload: serde_json::Value,
) -> Result<(), String> {
    // Set the timeout duration
    let timeout_duration = Duration::from_secs(30);

    // Spawn the blocking task with a timeout
    let handle_task = tokio::task::spawn_blocking(move || -> Result<(), String> {
        let game_state = payload
            .try_into()
            .map_err(|_| "Failed to parse game state")?;

        {
            let agent_lock = agent.blocking_lock();

            match agent_lock.as_ref() {
                Some(agent) => Ok(agent.on_game_ended(game_state)),
                None => Err("Agent not initialized".to_string()),
            }
        }
        .map_err(|e| format!("Failed to get agent response, {}", e))?;

        Ok(())
    });

    // Apply the timeout
    match timeout(timeout_duration, handle_task).await {
        Ok(Ok(result)) => result?,
        Ok(Err(e)) => return Err(format!("Task failed: {}", e)),
        Err(_) => return Err("Task timed out".to_string()),
    };

    Ok(())
}
