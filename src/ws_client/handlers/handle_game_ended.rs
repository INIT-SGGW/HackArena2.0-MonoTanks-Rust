use crate::bot::Bot;
use crate::bot_trait::BotTrait;
use crate::ws_client::packet::packets::game_end::game_end::GameEnd;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_game_ended(
    bot: Arc<Mutex<Option<Bot>>>,
    game_end: GameEnd,
) -> Result<(), String> {
    let result = {
        let bot_lock = bot.lock().await;

        match bot_lock.as_ref() {
            Some(bot) => Ok(bot.on_game_ended(game_end)),
            None => Err("Bot not initialized".to_string()),
        }
    };

    result.map_err(|e| format!("Failed to get bot response, {}", e))?;

    Ok(())
}
