use crate::agent::MyAgent;
use crate::game::agent_trait::Agent;
use crate::ws_client::packet::dto::raw_game_state::RawGameState;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;

pub async fn handle_next_move(
    tx: Sender<Message>,
    agent: Arc<Mutex<Option<MyAgent>>>,
    raw_game_state: RawGameState,
) -> Result<(), String> {
    // Spawn the blocking task with a timeout
    let game_state = raw_game_state
        .try_into()
        .map_err(|e| format!("Failed to parse into game state, {}", e))?;

    // Let's do this in separate scope to release the lock as soon as possible
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

    let response = serde_json::to_string(&agent_response).map_err(|e| e.to_string())?;

    println!("🤖 Agent response: {}", response);

    // Send the response
    tx.send(Message::Text(response))
        .await
        .map_err(|_| "Failed to send message")?;

    Ok(())
}
