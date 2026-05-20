mod connection;
mod info;
mod list;
mod publish;
mod start;
mod stop;
mod time;

pub(crate) use time::parse_time;
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
    /// Start tracking time on a ticket
    Start(StartArgs),
    /// Stop tracking time on active ticket
    Stop(StopArgs),
    /// List slots
    List(ListArgs),
    /// Publish slots to backend
    Publish(PublishArgs),
    /// Manage connections
    Connection {
        #[command(subcommand)]
        command: Connection,
    },
}

#[derive(Args, Debug)]
struct StartArgs {
    /// Ticket key (e.g. DES-1234). Omit for interactive mode.
    ticket: Option<String>,
    /// Start time (e.g. "14:30" or "2026-05-19 14:30"). Requires ticket.
    #[arg(long, requires = "ticket")]
    at: Option<String>,
    /// Note attached to this slot. Requires ticket.
    #[arg(long, requires = "ticket")]
    note: Option<String>,
}

#[derive(Args, Debug)]
struct StopArgs {
    /// Stop time (e.g. "14:30" or "2026-05-19 14:30")
    #[arg(long)]
    at: Option<String>,
}

#[derive(Args, Debug)]
struct ListArgs {
    /// Order by ticket instead of time
    #[arg(long)]
    by_ticket: bool,
    /// Range start (e.g. "14:30" or "2026-05-19 14:30"). Defaults to today 00:00.
    #[arg(long)]
    start: Option<String>,
    /// Range stop (e.g. "14:30" or "2026-05-19 14:30"). Unbounded if omitted.
    #[arg(long)]
    stop: Option<String>,
}

#[derive(Args, Debug)]
struct PublishArgs {
    /// Range start (e.g. "14:30" or "2026-05-19 14:30"). Defaults to today 00:00.
    #[arg(long)]
    start: Option<String>,
    /// Range stop (e.g. "14:30" or "2026-05-19 14:30"). Unbounded if omitted.
    #[arg(long)]
    stop: Option<String>,
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
    /// Test active connection
    Test,
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
        Some(Command::Start(args)) => start::run(args),
        Some(Command::Stop(args)) => stop::run(args),
        Some(Command::List(args)) => list::run(args),
        Some(Command::Publish(args)) => publish::run(args),
        Some(Command::Connection { command }) => match command {
            Connection::Add(args) => connection::add(args),
            Connection::List => connection::list(),
            Connection::Use { query } => connection::activate(query.as_deref()),
            Connection::Test => connection::test(),
        },
        None => Ok(()),
    }
}
