use crate::agent::Agent;
use crate::agent_trait::AgentTrait;
use crate::ws_client::packet::packet::Warning;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_on_warning_received(
    agent: Arc<Mutex<Option<Agent>>>,
    warning: Warning,
) -> Result<(), String> {
    // Let's do this in separate scope to release the lock as soon as possible
    {
        let mut agent_lock = agent
            .try_lock()
            .map_err(|_| "Failed to lock agent, it is already in use")?;

        match agent_lock.as_mut() {
            Some(agent) => Ok(agent.on_warning_received(warning)),
            None => Err("Agent not initialized".to_string()),
        }
    }
    .map_err(|e| format!("Failed to get agent response, {}", e))?;

    Ok(())
}
