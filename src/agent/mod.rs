use crate::game::agent_trait::Agent;
use crate::ws_client::packet::dto::game_end::GameEnd;
use crate::ws_client::packet::dto::{
    game_state::GameState, lobby_data::LobbyData, move_direction::MoveDirection, rotation::Rotation,
};
use crate::ws_client::packet::packet::AgentResponse;

pub struct MyAgent {
    my_id: String,
}

impl Agent for MyAgent {
    fn new(lobby_data: LobbyData) -> Self
    where
        Self: Sized,
    {
        MyAgent {
            my_id: lobby_data.player_id,
        }
    }

    fn next_move(&mut self, game_state: GameState) -> AgentResponse {
        let r = rand::random::<f32>();
        // println!("Random value for main match: {}", r);

        match r {
            r if r < 0.33 => {
                let direction = if rand::random::<bool>() {
                    MoveDirection::Forward
                } else {
                    MoveDirection::Backward
                };

                // println!("Selected MoveDirection: {:?}", direction);
                AgentResponse::TankMovement { direction }
            }
            r if r < 0.66 => {
                let random_rotation = || {
                    let rr = rand::random::<f32>();
                    // println!("Random value for rotation match: {}", rr);
                    match rr {
                        rr if rr < 0.33 => Some(Rotation::Left),
                        rr if rr < 0.66 => Some(Rotation::Right),
                        _ => None,
                    }
                };

                let tank_rotation = random_rotation();
                let turret_rotation = random_rotation();
                // println!(
                //     "Selected TankRotation: {:?}, TurretRotation: {:?}",
                //     tank_rotation, turret_rotation
                // );

                AgentResponse::TankRotation {
                    tank_rotation,
                    turret_rotation,
                }
            }
            _ => {
                // println!("Selected TankShoot");
                AgentResponse::TankShoot
            }
        }
    }

    fn on_game_ended(&self, game_end: GameEnd) {
        println!("Game ended");
        let winner = game_end
            .players
            .iter()
            .max_by_key(|player| player.score.unwrap())
            .unwrap();

        if winner.id == self.my_id {
            println!("I won!");
        }

        game_end.players.iter().for_each(|player| {
            println!(
                "Player: {} - Score: {}",
                player.nickname,
                player.score.unwrap()
            );
        });
    }
}
