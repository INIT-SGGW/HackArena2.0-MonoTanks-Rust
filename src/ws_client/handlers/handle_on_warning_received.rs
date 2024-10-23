use crate::bot::Bot;
use crate::bot_trait::BotTrait;
use crate::ws_client::packet::warning::Warning;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_on_warning_received(
    bot: Arc<Mutex<Option<Bot>>>,
    warning: Warning,
) -> Result<(), String> {
    // Let's do this in separate scope to release the lock as soon as possible
    {
        let mut bot_lock = bot
            .try_lock()
            .map_err(|_| "Failed to lock bot, it is already in use")?;

        match bot_lock.as_mut() {
            Some(bot) => Ok(bot.on_warning_received(warning)),
            None => Err("Bot not initialized".to_string()),
        }
    }
    .map_err(|e| format!("Failed to get bot response, {}", e))?;

    Ok(())
}
