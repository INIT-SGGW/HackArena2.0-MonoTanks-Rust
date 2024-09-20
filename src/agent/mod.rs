use crate::game::{agent_response::{move_direction::MoveDirection, rotation::Rotation}, lobby_data::LobbyData};
pub use crate::game::{
    agent_response::AgentResponsePayload, agent_trait::Agent,
    game_state::GameState,
};

pub struct MyAgent {
}

impl Agent for MyAgent {
    fn new(lobby_data: LobbyData) -> Self
    where
        Self: Sized,
    {
        MyAgent {  }
    }

    fn next_move(&mut self, game_state: GameState) -> AgentResponsePayload {
        match rand::random::<f32>() {
            r if r < 0.33 => {
                let direction = if rand::random::<bool>() {
                    MoveDirection::Forward
                } else {
                    MoveDirection::Backward
                };

                AgentResponsePayload::Move { direction }
            }
            r if r < 0.66 => {
                let random_rotation = || match rand::random::<f32>() {
                    r if r < 0.33 => Some(Rotation::Left),
                    r if r < 0.66 => Some(Rotation::Right),
                    _ => None,
                };

                AgentResponsePayload::Rotation {
                    tank_rotation: random_rotation(),
                    turret_rotation: random_rotation(),
                }
            }
            _ => AgentResponsePayload::Shoot,
        }
    }

    fn on_game_ended(&self, _game_state: GameState) {}
}
