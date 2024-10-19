use crate::agent_trait::AgentTrait;
use crate::ws_client::packet::packets::agent_response::ability_type::AbilityType;
use crate::ws_client::packet::packets::agent_response::agent_response::AgentResponse;
use crate::ws_client::packet::packets::agent_response::move_direction::MoveDirection;
use crate::ws_client::packet::packets::agent_response::rotation::Rotation;
use crate::ws_client::packet::packets::game_end::game_end::GameEnd;
use crate::ws_client::packet::packets::game_state::game_state::GameState;
use crate::ws_client::packet::packets::game_state::tile::tile::TileEntity;
use crate::ws_client::packet::packets::lobby_data::LobbyData;
use crate::ws_client::packet::warning::Warning;

pub struct Agent {
    my_id: String,
}

impl AgentTrait for Agent {
    /// Called when the agent joins a lobby, creating a new instance of the agent.
    /// This method initializes the agent with the lobby's current state and
    /// other relevant details.
    ///
    /// # Parameters
    /// - `lobby_data`: The initial state of the lobby when the agent joins.
    ///   Contains information like player data, game settings, etc.
    ///
    /// # Returns
    /// - A new instance of the agent.
    fn on_joining_lobby(lobby_data: LobbyData) -> Self
    where
        Self: Sized,
    {
        Agent {
            my_id: lobby_data.player_id,
        }
    }

    /// Called whenever there is a change in the lobby data.
    ///
    /// This method is triggered under various circumstances, such as:
    /// - When a player joins or leaves the lobby.
    /// - When server-side game settings are updated.
    ///
    /// # Parameters
    /// - `lobby_data`: The updated state of the lobby, containing information
    ///   like player details, game configurations, and other relevant data.
    ///   This is the same data structure as the one provided when the agent
    ///   first joined the lobby.
    ///
    /// # Default Behavior
    /// By default, this method performs no action. To add custom behavior
    /// when the lobby state changes, override this method in your implementation.
    fn on_lobby_data_changed(&mut self, lobby_data: LobbyData) {
        let _ = lobby_data;
    }

    /// Called when the game is about to start.
    ///
    /// This method is invoked just before the game begins.
    ///
    /// # Default Behavior
    /// By default, this method performs no action. Override this method to
    /// implement custom behavior for your agent when the game is starting.
    ///
    /// # Notes
    /// - This method is called after `on_joining_lobby` and before the first
    ///   `next_move` call.
    fn on_game_starting(&self) {}

    /// Called after each game tick, when new game state data is received from the server.
    /// This method is responsible for determining the agent's next move based on the
    /// current game state.
    ///
    /// # Parameters
    /// - `game_state`: The current state of the game, which includes all
    ///   necessary information for the agent to decide its next action,
    ///   such as the entire map with walls, tanks, bullets, zones, etc.
    ///
    /// # Returns
    /// - `AgentResponse`: The action or decision made by the agent, which will
    ///   be communicated back to the game server.
    fn next_move(&mut self, game_state: GameState) -> AgentResponse {
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
            r if r < 0.75 => {
                let random_number = rand::random::<f32>();

                let ability_type = if random_number < 0.20 {
                    AbilityType::DropMine
                } else if random_number < 0.40 {
                    AbilityType::FireBullet
                } else if random_number < 0.60 {
                    AbilityType::FireDoubleBullet
                } else if random_number < 0.80 {
                    AbilityType::UseLaser
                } else {
                    AbilityType::UseRadar
                };

                AgentResponse::AbilityUse { ability_type }
            }
            _ => AgentResponse::Pass,
        }
    }

    /// Called when a warning is received from the server.
    /// Please, do remember that if you agent is stuck on processing warning,
    /// the next move won't be called and vice versa.
    ///
    /// # Parameters
    /// - `warning`: The warning received from the server.
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
    /// implement any post-game behavior, such as logging, updating agent strategies,
    /// or other clean-up tasks.
    ///
    /// # Notes
    /// - This method is optional to override, but it can be useful for handling
    ///   game result analysis and logging.
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
