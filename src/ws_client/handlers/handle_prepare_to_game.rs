use crate::agent::Agent;
use crate::game::agent_trait::AgentTrait;
use crate::ws_client::packet::dto::lobby_data::LobbyData;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;

pub async fn handle_prepare_to_game(
    _tx: tokio::sync::mpsc::Sender<Message>,
    agent: Arc<Mutex<Option<Agent>>>,
    lobby_data: LobbyData,
) -> Result<(), String> {
    let mut agent_guard = agent.lock().await;

    if let None = *agent_guard {
        *agent_guard = Some(Agent::new(lobby_data));
        println!("🤖 Created agent");
    }

    Ok(())
}
