mod connection;
mod info;
use clap::{Args, Parser, Subcommand, ValueEnum};
use std::fmt;
use std::fmt::Display;

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
    /// Manage connections
    Connection {
        #[command(subcommand)]
        command: Connection,
    },
}

#[derive(Subcommand)]
enum Connection {
    /// Add a new connection
    Add(AddArgs),
    /// List all connections
    List,
    /// Change active connection
    Use {
        /// Connection ID or name (omit for interactive selection)
        query: Option<String>,
    },
}

#[derive(Args, Debug)]
struct AddArgs {
    #[arg(long, requires_all = ["kind", "host", "identity", "storage", "token"])]
    name: Option<String>,
    #[arg(long, requires_all = ["name", "host", "identity", "storage", "token"])]
    kind: Option<ConnectionKind>,
    #[arg(long, requires_all = ["name", "kind", "identity", "storage", "token"])]
    host: Option<String>,
    #[arg(long, requires_all = ["name", "kind", "host", "storage", "token"])]
    identity: Option<String>,
    #[arg(long, requires_all = ["name", "kind", "host", "identity", "token"])]
    storage: Option<SecretStore>,
    #[arg(long, requires_all = ["name", "kind", "host", "identity", "storage"])]
    token: Option<String>,
}

#[derive(Clone, ValueEnum, Debug)]
pub(crate) enum ConnectionKind {
    JiraCloud,
    JiraDc,
}

impl Display for ConnectionKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kind = match self {
            Self::JiraCloud => "jira-cloud",
            Self::JiraDc => "jira-dc",
        };
        write!(f, "{}", kind,)
    }
}

#[derive(Clone, ValueEnum, Debug)]
pub(crate) enum SecretStore {
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    Keyring,
    Plaintext,
}

impl Display for SecretStore {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kind = match self {
            #[cfg(any(target_os = "macos", target_os = "windows"))]
            Self::Keyring => "keyring",
            Self::Plaintext => "plaintext",
        };
        write!(f, "{}", kind,)
    }
}

pub(crate) fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Info) => info::run(),
        Some(Command::Connection { command }) => match command {
            Connection::Add(args) => connection::add(args),
            Connection::List => connection::list(),
            Connection::Use { query } => connection::activate(query.as_deref()),
        },
        None => Ok(()),
    }
}
