mod connection;
mod info;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "nirvana",
    version,
    about = "App to get you to time-tracking nirvana"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Show system and app diagnostics
    Info,
    /// Backend connection management
    Connection {
        #[command(subcommand)]
        command: Connection,
    },
}

#[derive(Subcommand)]
enum Connection {
    /// List all connections
    List,
    /// Change active connection
    Use {
        /// Connection ID or name (omit for interactive selection)
        query: Option<String>,
    },
}

pub(crate) fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Info) => info::run(),
        Some(Command::Connection { command }) => match command {
            Connection::List => connection::list(),
            Connection::Use { query } => connection::activate(query.as_deref()),
        },
        None => Ok(()),
    }
}
