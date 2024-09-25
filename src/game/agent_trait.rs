use crate::ws_client::packet::dto::{
    game_end::GameEnd, game_state::GameState, lobby_data::LobbyData,
};

use super::agent_response::AgentResponse;

pub trait AgentTrait: Send + Sync {
    fn new(game_info: LobbyData) -> Self
    where
        Self: Sized;

    fn next_move(&mut self, game_state: GameState) -> AgentResponse;

    fn on_game_ended(&self, _game_end: GameEnd) {}
}
