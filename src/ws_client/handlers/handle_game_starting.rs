use crate::agent::Agent;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;
use crate::agent_trait::AgentTrait;
use crate::ws_client::packet::packet::Packet;

pub async fn handle_game_starting(
    tx: Sender<Message>,
    agent: Arc<Mutex<Option<Agent>>>
) -> Result<(), String> {
    let result = {
        let agent_lock = agent.lock().await;

        match agent_lock.as_ref() {
            Some(agent) => Ok(agent.on_game_starting()),
            None => Err("Agent not initialized".to_string()),
        }
    };

    result.map_err(|e| format!("Failed to get agent response, {}", e))?;

    tx.send(Message::Text(Packet::ReadyToReceiveGameState.into()))
        .await
        .map_err(|_| "Failed to send message")?;

    Ok(())
}
