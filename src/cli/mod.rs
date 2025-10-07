pub mod config;

#[derive(clap::Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
    #[arg(short = 'c', long = "config", default_value = "domains.toml")]
    pub config_path: String,
}

#[derive(clap::Subcommand, PartialEq, Eq, PartialOrd, Ord)]
pub enum Command {
    LIST { domain: String },
    PING,
    UPDATE,
}
