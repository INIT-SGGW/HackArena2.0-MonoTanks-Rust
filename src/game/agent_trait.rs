use super::{agent_response::AgentResponsePayload, game_state::GameState, lobby_data::LobbyData};

pub trait Agent: Send + Sync {
    fn new(game_info: LobbyData) -> Self
    where
        Self: Sized;

    fn next_move(&mut self, game_state: GameState) -> AgentResponsePayload;

    fn on_game_ended(&self, _game_state: GameState) {}
}
