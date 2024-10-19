use crate::agent::Agent;
use crate::agent_trait::AgentTrait;
use crate::ws_client::packet::packet::Packet;
use crate::ws_client::packet::packets::lobby_data::LobbyData;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;

pub async fn handle_prepare_to_game(
    tx: tokio::sync::mpsc::Sender<Message>,
    agent: Arc<Mutex<Option<Agent>>>,
    lobby_data: LobbyData,
) -> Result<(), String> {
    let mut agent_guard = agent.lock().await;

    match agent_guard.as_mut() {
        Some(agent) => agent.on_lobby_data_changed(lobby_data),
        None => {
            let sandbox_mode = lobby_data.server_settings.sandbox_mode;

            *agent_guard = Some(Agent::on_joining_lobby(lobby_data));
            println!("[System] 🤖 Created agent");

            if sandbox_mode {
                println!("[System] 🛠️ Sandbox mode enabled");

                match tx
                    .send(Message::Text(Packet::ReadyToReceiveGameState.into()))
                    .await
                {
                    Ok(_) => println!("[System] 🎳 Ready to receive game state sent"),
                    Err(e) => {
                        eprintln!("[System] 🚨 Error sending ReadyToReceiveGameState -> {}", e)
                    }
                }

                tx.send(Message::Text(Packet::GameStatusRequest.into()))
                    .await
                    .map_err(|e| format!("🚨 Error sending GameStatusRequest -> {}", e))?;
            }
        }
    }

    Ok(())
}
