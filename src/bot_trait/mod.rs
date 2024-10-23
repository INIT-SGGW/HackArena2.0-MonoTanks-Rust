use crate::ws_client::packet::packets::bot_response::bot_response::BotResponse;
use crate::ws_client::packet::packets::game_end::game_end::GameEnd;
use crate::ws_client::packet::packets::game_state::game_state::GameState;
use crate::ws_client::packet::packets::lobby_data::LobbyData;
use crate::ws_client::packet::warning::Warning;

/// A trait that defines the behavior of an AI bot interacting with the game
/// by responding to game state updates and making decisions based on the current state.
pub trait BotTrait: Send + Sync {
    /// Called when the bot joins a lobby, creating a new instance of the bot.
    /// This method initializes the bot with the lobby's current state and
    /// other relevant details.
    ///
    /// # Parameters
    /// - `lobby_data`: The initial state of the lobby when the bot joins.
    ///   Contains information like player data, game settings, etc.
    ///
    /// # Returns
    /// - A new instance of the bot.
    fn on_joining_lobby(lobby_data: LobbyData) -> Self
    where
        Self: Sized;

    /// Called whenever there is a change in the lobby data.
    ///
    /// This method is triggered under various circumstances, such as:
    /// - When a player joins or leaves the lobby.
    /// - When server-side game settings are updated.
    ///
    /// # Parameters
    /// - `lobby_data`: The updated state of the lobby, containing information
    ///   like player details, game configurations, and other relevant data.
    ///   This is the same data structure as the one provided when the bot
    ///   first joined the lobby.
    ///
    /// # Default Behavior
    /// By default, this method performs no action. To add custom behavior
    /// when the lobby state changes, override this method in your implementation.
    fn on_lobby_data_changed(&mut self, lobby_data: LobbyData) {
        let _ = lobby_data;
    }

    /// Called after each game tick, when new game state data is received from the server.
    /// This method is responsible for determining the bot's next move based on the
    /// current game state.
    ///
    /// # Parameters
    /// - `game_state`: The current state of the game, which includes all
    ///   necessary information for the bot to decide its next action,
    ///   such as the entire map with walls, tanks, bullets, zones, etc.
    ///
    /// # Returns
    /// - `BotResponse`: The action or decision made by the bot, which will
    ///   be communicated back to the game server.
    fn next_move(&mut self, game_state: GameState) -> BotResponse;

    /// Called when a warning is received from the server.
    /// Please, do remember that if your bot is stuck on processing warning,
    /// the next move won't be called and vice versa.
    ///
    /// # Parameters
    /// - `warning`: The warning received from the server.
    fn on_warning_received(&mut self, warning: Warning) {
        let _ = warning;
    }

    /// Called when the game has concluded, providing the final game results.
    ///
    /// This method is triggered when the game ends, which is when a defined
    /// number of ticks in LobbyData has passed.
    ///
    /// # Parameters
    /// - `game_end`: The final state of the game, containing players' scores.
    ///
    /// # Default Behavior
    /// By default, this method performs no action. You can override it to
    /// implement any post-game behavior, such as logging, or other clean-up tasks.
    ///
    /// # Notes
    /// - This method is optional to override, but it can be useful for handling
    ///   game result analysis and logging.
    fn on_game_ended(&self, game_end: GameEnd) {
        let _ = game_end;
    }
}
