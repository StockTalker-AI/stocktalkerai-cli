use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "stocktalkerai", author = "Tomi", version, about = "CLI for StockTalker AI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Output format (table or json)
    #[arg(short, long, default_value = "table", global = true)]
    pub output: String,

    /// API Key (or set STOCKTALKERAI_API_KEY env var)
    #[arg(long, env = "STOCKTALKERAI_API_KEY", global = true, hide_env_values = true)]
    pub api_key: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Check API status
    Status,
    /// Get user account info and active alerts count
    Account,
    /// Get current price of a ticker
    Price {
        ticker: String,
        /// Get 30-day price history
        #[arg(long)]
        history: bool,
    },
    /// Get technical indicators (SMA, RSI)
    Indicators {
        ticker: String,
    },
    /// Manage alerts
    Alert {
        #[command(subcommand)]
        action: AlertCommand,
    },
    /// Manage lists
    List {
        #[command(subcommand)]
        action: ListCommand,
    },
}

#[derive(Subcommand)]
pub enum ListCommand {
    /// Retrieve all alert lists
    Ls,
}

#[derive(Subcommand)]
pub enum AlertCommand {
    /// List all your alerts
    Ls {
        /// Filter by status (e.g. active, triggered)
        #[arg(long)]
        status: Option<String>,
        /// Filter by list ID
        #[arg(long)]
        list: Option<String>,
    },
    /// Get details of a specific alert
    Get {
        id: String,
    },
    /// Create a new alert using natural language
    Create {
        prompt: String,
        /// Optional ID of the list to attach this alert to
        #[arg(long)]
        list: Option<String>,
        /// Optional note to attach to this alert
        #[arg(long)]
        note: Option<String>,
        /// Pin this alert to the top of its list
        #[arg(long)]
        pin: bool,
        /// Assign a color to this alert (blue, green, yellow, red, purple)
        #[arg(long)]
        color: Option<String>,
        /// Return the full underlying nested alert object
        #[arg(long)]
        full: bool,
    },
    /// Edit an existing alert
    Edit {
        id: String,
        /// Update or remove list ID (pass "" to remove)
        #[arg(long)]
        list: Option<String>,
        /// Update or remove note (pass "" to remove)
        #[arg(long)]
        note: Option<String>,
        /// Pin this alert to the top of its list
        #[arg(long)]
        pin: bool,
        /// Unpin this alert
        #[arg(long)]
        unpin: bool,
        /// Assign a color to this alert (blue, green, yellow, red, purple)
        #[arg(long)]
        color: Option<String>,
        /// Remove color assignment from this alert
        #[arg(long)]
        clear_color: bool,
    },
    /// Archive an alert
    Archive {
        id: String,
    },
}
