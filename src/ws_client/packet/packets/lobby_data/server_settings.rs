use derive_more::derive::Constructor;
use serde::{Deserialize, Serialize};

/// Represents the configuration settings for the server.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash, Constructor)]
#[serde(rename_all = "camelCase")]
pub struct ServerSettings {
    /// The dimensions of the grid. The grid is a square with sides of equal length.
    pub grid_dimension: u32,

    /// The number of players participating in the game. Minimum is 2. Maximum is 4.
    pub number_of_players: u32,

    /// The seed value used for random number generation, ensuring consistency in results.
    /// It is used to generate the grid and player starting positions.
    pub seed: u32,

    /// The interval at which broadcast messages are sent to bots, in milliseconds.
    pub broadcast_interval: u32,

    /// A flag that determines whether broadcasts should happen
    /// immediately after all players have made their action (true)
    /// or at regular intervals (false).
    pub eager_broadcast: bool,

    /// Indicates whether the game is running in sandbox mode.
    pub sandbox_mode: bool,

    /// The current tick of the game. This is None in sandbox mode.
    pub tick: Option<u32>,

    /// The name of the match.
    pub match_name: Option<String>,

    /// The version of the game running on the server.
    pub version: String,
}
