use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
pub struct Args {
    #[clap(short, long)]
    pub nickname: String,

    #[clap(long, default_value = "localhost")]
    pub host: String,

    #[clap(short, long, default_value = "5000")]
    pub port: u16,

    #[clap(short, long, default_value = "")]
    pub code: String,

    #[clap(short, long, default_value = "false")]
    pub debug_quick_join: bool,
}
