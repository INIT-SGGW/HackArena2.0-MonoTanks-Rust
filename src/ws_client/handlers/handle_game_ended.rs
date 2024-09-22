use crate::agent::MyAgent;
use crate::game::agent_trait::Agent;
use crate::ws_client::packet::dto::game_state::GameState;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_game_ended(
    agent: Arc<Mutex<Option<MyAgent>>>,
    payload: GameState,
) -> Result<(), String> {
    let game_state = payload
        .try_into()
        .map_err(|_| "Failed to parse game state")?;

    let result = {
        let agent_lock = agent.blocking_lock();

        match agent_lock.as_ref() {
            Some(agent) => Ok(agent.on_game_ended(game_state)),
            None => Err("Agent not initialized".to_string()),
        }
    };

    result.map_err(|e| format!("Failed to get agent response, {}", e))?;

    Ok(())
}
