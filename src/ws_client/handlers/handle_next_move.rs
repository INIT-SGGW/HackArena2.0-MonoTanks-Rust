use crate::bot::Bot;
use crate::bot_trait::BotTrait;
use crate::ws_client::packet::packets::game_state::raw_game_state::RawGameState;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::Message;

pub async fn handle_next_move(
    tx: Sender<Message>,
    bot: Arc<Mutex<Option<Bot>>>,
    raw_game_state: RawGameState,
) -> Result<(), String> {
    let game_state_id = raw_game_state.id.clone();

    // Spawn the blocking task with a timeout
    let game_state = raw_game_state
        .try_into()
        .map_err(|e| format!("Failed to parse into game state, {}", e))?;

    // Let's do this in separate scope to release the lock as soon as possible
    let bot_response = {
        let mut bot_lock = bot
            .try_lock()
            .map_err(|_| "Failed to lock bot, it is already in use")?;

        match bot_lock.as_mut() {
            Some(bot) => Ok(bot.next_move(game_state)),
            None => Err("Bot not initialized".to_string()),
        }
    }
    .map_err(|e| format!("Failed to get bot response, {}", e))?;

    let response_packet = bot_response.to_packet(game_state_id);
    let response_string = serde_json::to_string(&response_packet).map_err(|e| e.to_string())?;

    // Send the response
    tx.send(Message::Text(response_string))
        .await
        .map_err(|_| "Failed to send message")?;

    Ok(())
}
