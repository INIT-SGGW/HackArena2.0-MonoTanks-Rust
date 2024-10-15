use crate::agent_trait::AgentTrait;
use crate::ws_client::packet::packet::Warning;
use crate::ws_client::packet::packets::agent_response::ability_type::AbilityType;
use crate::ws_client::packet::packets::agent_response::agent_response::AgentResponse;
use crate::ws_client::packet::packets::agent_response::move_direction::MoveDirection;
use crate::ws_client::packet::packets::agent_response::rotation::Rotation;
use crate::ws_client::packet::packets::game_end::game_end::GameEnd;
use crate::ws_client::packet::packets::game_state::game_state::GameState;
use crate::ws_client::packet::packets::game_state::tile::tile::TileEntity;
use crate::ws_client::packet::packets::lobby_data::LobbyData;

pub struct Agent {
    my_id: String,
}

impl AgentTrait for Agent {
    fn on_joining_lobby(lobby_data: LobbyData) -> Self
    where
        Self: Sized,
    {
        Agent {
            my_id: lobby_data.player_id,
        }
    }

    fn on_lobby_data_changed(&mut self, lobby_data: LobbyData) {
        let _ = lobby_data;
    }

    fn next_move(&mut self, game_state: GameState) -> AgentResponse {
        // Print map as ascii
        println!("Map:");
        for row in game_state.map.iter() {
            for col in row.iter() {
                let entities = &col.entities;
                let symbol = {
                    if entities.iter().any(|entity| entity.is_wall()) {
                        '#'
                    } else if entities.iter().any(|entity| {
                        if let TileEntity::Tank(tank) = entity {
                            tank.owner_id == self.my_id
                        } else {
                            false
                        }
                    }) {
                        'T'
                    } else if entities.iter().any(|entity| entity.is_tank()) {
                        't'
                    } else if entities.iter().any(|entity| entity.is_bullet()) {
                        'B'
                    } else if entities.iter().any(|entity| entity.is_laser()) {
                        'L'
                    } else if entities.iter().any(|entity| entity.is_mine()) {
                        'M'
                    } else if entities.iter().any(|entity| entity.is_item()) {
                        'I'
                    } else if col.visible {
                        '.'
                    } else if col.zone_index.is_some() {
                        col.zone_index.unwrap() as char
                    } else {
                        ' '
                    }
                };

                print!(" {}", symbol);
            }
            println!();
        }

        // Find my tank
        let my_tank = game_state.map.iter().flatten().find(|tile| {
            tile.entities.iter().any(|obj| {
                if let TileEntity::Tank(tank) = obj {
                    tank.owner_id == self.my_id
                } else {
                    false
                }
            })
        });

        // If our tank is not found, it is dead, and we should pass
        if my_tank.is_none() {
            return AgentResponse::Pass;
        }

        // Do a random action
        match rand::random::<f32>() {
            r if r < 0.25 => {
                let direction = if rand::random::<bool>() {
                    MoveDirection::Forward
                } else {
                    MoveDirection::Backward
                };

                AgentResponse::Movement { direction }
            }
            r if r < 0.50 => {
                let random_rotation = || match rand::random::<f32>() {
                    r if r < 0.33 => Some(Rotation::Left),
                    r if r < 0.66 => Some(Rotation::Right),
                    _ => None,
                };

                AgentResponse::Rotation {
                    tank_rotation: random_rotation(),
                    turret_rotation: random_rotation(),
                }
            }
            r if r < 0.75 => AgentResponse::AbilityUse {
                ability_type: AbilityType::FireBullet,
            },
            _ => AgentResponse::Pass,
        }
    }

    fn on_warning_received(&mut self, warning: Warning) {
        match warning {
            Warning::PlayerAlreadyMadeActionWarning => {
                println!("⚠️ Player already made action warning")
            }
            Warning::MissingGameStateIdWarning => println!("⚠️ Missing game state id warning"),
            Warning::SlowResponseWarning => println!("⚠️ Slow response warning"),
            Warning::ActionIgnoredDueToDeadWarning => {
                println!("⚠️ Action ignored due to dead warning")
            }
            Warning::CustomWarning { message } => println!("⚠️ Custom warning: {}", message),
        }
    }

    fn on_game_ended(&self, game_end: GameEnd) {
        let winner = game_end
            .players
            .iter()
            .max_by_key(|player| player.score)
            .unwrap();

        if winner.id == self.my_id {
            println!("I won!");
        }

        game_end.players.iter().for_each(|player| {
            println!("Player: {} - Score: {}", player.nickname, player.score);
        });
    }
}
