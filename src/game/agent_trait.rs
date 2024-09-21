// use super::agent_response::AgentResponsePayload;
use crate::ws_client::packet::{
    dto::{game_state::GameState, lobby_data::LobbyData},
    packet::AgentResponse,
};

pub trait Agent: Send + Sync {
    fn new(game_info: LobbyData) -> Self
    where
        Self: Sized;

    fn next_move(&mut self, game_state: GameState) -> AgentResponse;

    fn on_game_ended(&self, _game_state: GameState) {}
}
