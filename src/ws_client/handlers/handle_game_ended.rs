use crate::agent::MyAgent;
use crate::game::agent_trait::Agent;
use crate::ws_client::packet::dto::game_end::GameEnd;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_game_ended(
    agent: Arc<Mutex<Option<MyAgent>>>,
    game_end: GameEnd,
) -> Result<(), String> {
    let result = {
        let agent_lock = agent.lock().await;

        match agent_lock.as_ref() {
            Some(agent) => Ok(agent.on_game_ended(game_end)),
            None => Err("Agent not initialized".to_string()),
        }
    };

    result.map_err(|e| format!("Failed to get agent response, {}", e))?;

    Ok(())
}
