# Rust WebSocket Client for Hackathon 2024

This Rust-based WebSocket client was developed for the Hackathon 2024, organized
by WULS-SGGW. It serves as a framework for participants to create AI agents that
can play the game.

To fully test and run the game, you will also need the game server and GUI
client, as the GUI provides a visual representation of gameplay. You can find
more information about the server and GUI client in the following repository:

- [Server and GUI Client Repository](https://github.com/INIT-SGGW/HackArena2024H2-Game)

## Development

The agent logic you are going to implement is located in `src/agent/mod.rs`:

```rust
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

    fn on_lobby_data_changed(&mut self, lobby_data: LobbyData) {}

    fn next_move(&mut self, game_state: GameState) -> AgentResponse {
        match rand::random::<f32>() {
            r if r < 0.25 => {
                let direction = if rand::random::<bool>() {
                    MoveDirection::Forward
                } else {
                    MoveDirection::Backward
                };

                AgentResponse::TankMovement { direction }
            }
            r if r < 0.50 => {
                let random_rotation = || match rand::random::<f32>() {
                    r if r < 0.33 => Some(Rotation::Left),
                    r if r < 0.66 => Some(Rotation::Right),
                    _ => None,
                };

                AgentResponse::TankRotation {
                    tank_rotation: random_rotation(),
                    turret_rotation: random_rotation(),
                }
            }
            r if r < 0.75 => AgentResponse::TankShoot,
            _ => AgentResponse::ResponsePass,
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
```

The `Agent` struct implements the `AgentTrait`, which defines the agent's
behavior. The `new` function is called when the agent is created, and the
`next_move` function is called every game tick to determine the agent's next
move. The `on_game_ended` function is called when the game ends to provide the
final game state.

`next_move` returns an `AgentResponse` enum, which can be one of the following:

- `TankMovement { direction: MoveDirection }`: Move the tank forward or
  backward, where `MoveDirection` is an enum with the variants `Forward` and
  `Backward`.
- `TankRotation { tank_rotation: Option<Rotation>, turret_rotation: Option<Rotation> }`:
  Rotate the tank and turret left or right, where `Rotation` is an enum with the
  variants `Left` and `Right`.
- `TankShoot`: Shoot a projectile in the direction the turret is facing.

You can modify the mentioned file and create more files in the `src/agent`
directory. Do not modify any other files, as this may prevent us from running
your agent during the competition.

If you want to extend the functionality of the `GameState` struct or other
structs, use the Extension Trait pattern by creating your own trait and
implementing it for the structs or enums you want to extend.

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
cargo run -- --nickname rust
```

The `--nickname` argument is required and must be unique. For additional
configuration options, run:

```sh
cargo run -- --help
```

To build and run an optimized release version of the client, use:

```sh
cargo run --release -- --nickname rust
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
Code's integrated terminal, as if you were running the project locally.

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
   docker run --rm client --nickname rust --host host.docker.internal
   ```

If the server is running on your local machine, use the
`--host host.docker.internal` flag to connect the Docker container to your local
host.

If you are using a machine with ARM architecture, you should modify the
Dockerfile and change every occurrence of `x86_64` to `aarch64`.
