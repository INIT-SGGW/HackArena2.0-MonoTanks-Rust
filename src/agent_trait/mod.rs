use crate::ws_client::packet::packets::agent_response::agent_response::AgentResponse;
use crate::ws_client::packet::packets::game_end::game_end::GameEnd;
use crate::ws_client::packet::packets::game_state::game_state::GameState;
use crate::ws_client::packet::packets::lobby_data::LobbyData;

/// A trait that defines the behavior of an agent interacting with the game.
/// Implementors of this trait must provide logic for how the agent responds
/// to various game events, including lobby interactions, game progress, and the
/// conclusion of a game.
///
/// # Overview
/// The `AgentTrait` is designed to represent an AI or automated player in the game.
/// It provides methods to handle different stages of gameplay, allowing developers
/// to define how an agent will behave in different situations, such as joining
/// a lobby, responding to game state changes, or processing the end-of-game outcomes.
pub trait AgentTrait: Send + Sync {
    /// Called when the agent joins a lobby, creating a new instance of the agent.
    ///
    /// This method initializes the agent with the lobby's current state and
    /// other relevant details.
    ///
    /// # Parameters
    /// - `_lobby_data`: The initial state of the lobby when the agent joins.
    ///   Contains information like player data, game settings, etc.
    ///
    /// # Returns
    /// - A new instance of the agent.
    ///
    /// # Notes
    /// - This method must be implemented by the agent, as it sets up the agent's
    ///   state upon entering a lobby.
    fn on_joining_lobby(_lobby_data: LobbyData) -> Self
    where
        Self: Sized;

    /// Called whenever there is a change in the lobby data.
    ///
    /// This method is triggered under various circumstances, such as:
    /// - When a player joins or leaves the lobby.
    /// - When server-side game settings are updated.
    ///
    /// # Parameters
    /// - `_lobby_data`: The updated state of the lobby, containing information
    ///   like player details, game configurations, and other relevant data.
    ///
    /// # Default Behavior
    /// By default, this method performs no action. To add custom behavior
    /// when the lobby state changes, override this method in your implementation.
    ///
    /// # Notes
    /// - Override this method if you need the agent to react to lobby changes,
    ///   such as preparing for game start when the required number of players
    ///   is reached.
    fn on_lobby_data_changed(&mut self, _lobby_data: LobbyData) {}

    /// Called after each game tick, when new game state data is received from the server.
    ///
    /// This method is responsible for determining the agent's next move based on the
    /// current game state.
    ///
    /// # Parameters
    /// - `_game_state`: The current state of the game, which includes all
    ///   necessary information for the agent to decide its next action,
    ///   such as entire map with walls, tanks, bullets, zones, etc.
    ///
    /// # Returns
    /// - `AgentResponse`: The action or decision made by the agent, which will
    ///   be communicated back to the game server.
    ///
    /// # Notes
    /// - This method must be implemented by the agent, as it directly influences
    ///   how the agent behaves during the game. It is called each time a new game
    ///   state from the server is received. This happens every tick.
    fn next_move(&mut self, _game_state: GameState) -> AgentResponse;

    /// Called when the game has concluded, providing the final game results.
    ///
    /// This method is triggered when the game ends and the results are available,
    /// including details like the winner, final scores, and other post-game data.
    ///
    /// # Parameters
    /// - `_game_end`: The final state of the game, containing information such
    ///   as the game's outcome, player performance, and other relevant statistics.
    ///
    /// # Default Behavior
    /// By default, this method performs no action. You can override it to
    /// implement any post-game behavior, such as logging, updating agent strategies,
    /// or other clean-up tasks.
    ///
    /// # Notes
    /// - This method is optional to override, but it can be useful for handling
    ///   game result analysis, logging, or resetting the agent for the next game.
    fn on_game_ended(&self, _game_end: GameEnd) {}
}
