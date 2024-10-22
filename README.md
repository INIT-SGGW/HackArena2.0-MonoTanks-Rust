# Bot Template in Rust for HackArena 2024

This template for a Bot for MonoTanks game for the HackArena 2024,
organized by WULS-SGGW. Is is implemented as a WebSocket client written
in Rust programming language.

To fully test and run the game, you will also need the game server and GUI
client, as the GUI provides a visual representation of gameplay. You can find
more information about the server and GUI client in the following repository:

- [Server and GUI Client Repository](https://github.com/INIT-SGGW/HackArena2024H2-Game)

The guide to the game mechanics and hackathon rules can be found on the:
- [instruction page](https://github.com/INIT-SGGW/HackArena2024H2-Game/blob/main/README.md).

## Development

Clone this repo using git:
```sh
git clone https://github.com/INIT-SGGW/HackArena2024H2-Rust.git
```

or download the [zip file](https://github.com/INIT-SGGW/HackArena2024H2-Rust/archive/refs/heads/main.zip)
and extract it.

The agent logic you are going to implement is located in `src/agent/mod.rs`:

```rust
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
```

## Running the Client

You can run this client in three different ways: locally, within a VS Code
development container, or manually using Docker.

### 1. Running Locally

To run the client locally, you must have Rust 1.75 or later installed. Verify
your Rust version by running:

```sh
rustc --version
```

Assuming the game server is running on `localhost:5000` (refer to the server
repository's README for setup instructions), start the client by running:

```sh
cargo run -- --nickname TEAM_NAME
```

The `--nickname` argument is required and must be unique. For additional
configuration options, run:

```sh
cargo run -- --help
```

To build and run an optimized release version of the client, use:

```sh
cargo run --release -- --nickname TEAM_NAME
```

### 2. Running in a VS Code Development Container

To run the client within a VS Code development container, ensure you have Docker
and Visual Studio Code (VS Code) installed, along with the Dev Containers
extension.

Steps:

1. Open the project folder in VS Code.
2. If prompted, choose to reopen the project in a development container and wait
   for the setup to complete.
3. If not prompted, manually reopen the project in a container by:
   - Opening the command palette (`F1`)
   - Searching for and selecting `>Dev Containers: Reopen in Container`

Once the container is running, you can execute all necessary commands in VS
Code's integrated terminal, as if you were running the project locally. However,
if server is running on your local machine, you should use `host.docker.internal`
as a host.

So the command to run the client would be:

```sh
cargo run -- --host host.docker.internal --nickname TEAM_NAME
```

### 3. Running in a Docker Container (Manual Setup)

To run the client manually in a Docker container, ensure Docker is installed on
your system.

Steps:

1. Build the Docker image:
   ```sh
   docker build -t client .
   ```
2. Run the Docker container:
   ```sh
   docker run --rm client --nickname TEAM_NAME --host host.docker.internal
   ```

If the server is running on your local machine, use the
`--host host.docker.internal` flag to connect the Docker container to your local
host.

If you are using a machine with ARM architecture (like Apple M series processors),
you should modify the Dockerfile and change every occurrence of `x86_64` to
`aarch64`.

## FAQ

### What can we modify?

You can modify the `src/agent/mod.rs` file to implement your own agent logic
as well as create new files in the `src/agent` directory to implement additional
functionality. In case you would like to implement new methods on existing
structs, use the new type pattern or extension trait pattern.

You can add new crates to the `Cargo.toml` file, but do not delete or change
versions of the crates that are already present in the file.

Please, do not modify any other files, as they are used for proper network
communication with the game server.

### Can we include static files?

If you need to include static files that your program should access during
testing or execution, place them in the `data` folder. This folder is copied
into the Docker image and will be accessible to your application at runtime. For
example, you could include configuration files, pre-trained models, or any other
data your agent might need.

### In what format we will need to submit our bot?

You will need to submit a zip file containing the whole repository. Of course,
please, delete the `target` directory and any other temporary files before
submitting, so the file size is as small as possible.
