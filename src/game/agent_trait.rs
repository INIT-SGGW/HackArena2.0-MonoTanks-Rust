use super::{agent_response::AgentResponsePayload, game_info::GameInfo, game_state::GameState};

pub trait Agent: Send + Sync {
    fn new(game_info: GameInfo) -> Self
    where
        Self: Sized;

    fn next_move(&mut self, game_state: GameState) -> AgentResponsePayload;

    fn on_game_ended(&self, _game_state: GameState) {}
}
