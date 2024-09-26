use clap::Parser;

/// Command-line arguments for configuring the agent and connecting to a server.
///
/// This structure defines the available parameters that can be passed via
/// the command line to configure an agent in the game environment, including
/// its nickname, server address, port, and optional access code.
#[derive(Parser, Debug)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
pub struct Args {
    /// Nickname of the agent that will be displayed in the game.
    ///
    /// This must be a unique identifier for the agent in the game environment.
    /// Nicknames that are already in use or not unique will cause conflicts.
    #[clap(short, long)]
    pub nickname: String,

    /// The IP address or domain name of the server to connect to.
    ///
    /// The agent will attempt to establish a connection to the specified host.
    /// If not provided, it defaults to "localhost".
    #[clap(long, default_value = "localhost")]
    pub host: String,

    /// The port on which the server is listening.
    ///
    /// This specifies the port number that the server is using for communication.
    /// If not provided, it defaults to port 5000.
    #[clap(short, long, default_value = "5000")]
    pub port: u16,

    /// Optional access code required to join the server.
    ///
    /// If the server enforces an access code for connections, it must be supplied here.
    /// If no code is required, this can be left empty (default is an empty string).
    #[clap(short, long, default_value = "")]
    pub code: String,
}
