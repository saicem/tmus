use clap::{Parser, Subcommand, ValueEnum};
use tracing::Level;

mod app;
mod category;
mod output;

use crate::app::constant::{data_dir, APP_NAME};
use crate::cmd::read_by_timestamp;
use output::{format_duration, format_timestamp, print_json, print_table};
use tmus_engine::util::Timestamp;
use tmus_engine::{engine_start, EngineMode};

#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum LogLevel {
    #[default]
    #[value(name = "info")]
    Info,
    #[value(name = "trace")]
    Trace,
    #[value(name = "debug")]
    Debug,
    #[value(name = "warn")]
    Warn,
    #[value(name = "error")]
    Error,
}

impl From<LogLevel> for Level {
    fn from(log_level: LogLevel) -> Self {
        match log_level {
            LogLevel::Trace => Level::TRACE,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Info => Level::INFO,
            LogLevel::Warn => Level::WARN,
            LogLevel::Error => Level::ERROR,
        }
    }
}

#[derive(Parser)]
#[command(name = APP_NAME, version, about = "Application usage tracker")]
pub struct Cli {
    /// Output result as JSON instead of human-friendly text
    #[arg(long, global = true)]
    pub json: bool,

    /// Set log level (trace, debug, info, warn, error)
    #[arg(long, value_name = "LEVEL")]
    pub level: Option<LogLevel>,

    /// Start without window (daemon mode)
    #[arg(long)]
    pub nw: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Show current local time
    Time,
    /// List or manage tracked applications
    App {
        #[command(subcommand)]
        command: AppCommand,
    },
    /// Query app usage duration in a time range
    Usage(UsageArgs),
    /// Query raw focus records
    Records(RecordsArgs),
    /// Manage application categories
    Category {
        #[command(subcommand)]
        command: CategoryCommand,
    },
}

#[derive(clap::Args)]
pub struct UsageArgs {
    /// Start time: ISO-8601 (2026-08-01T00:00:00+08:00), date (2026-08-01) or unix milliseconds
    #[arg(long)]
    pub start: Option<String>,

    /// End time, same formats as --start (default: now)
    #[arg(long)]
    pub end: Option<String>,

    /// Return only the top N apps by duration
    #[arg(long)]
    pub top_k: Option<usize>,
}

#[derive(clap::Args)]
pub struct RecordsArgs {
    /// Start time: ISO-8601, date or unix milliseconds (default: 0)
    #[arg(short, long)]
    pub start: Option<String>,

    /// End time, same formats as --start (default: now)
    #[arg(short, long)]
    pub end: Option<String>,

    /// Filter by app ID
    #[arg(short, long)]
    pub app_id: Option<usize>,

    /// Limit number of records
    #[arg(short, long)]
    pub limit: Option<usize>,
}

#[derive(Subcommand)]
pub enum AppCommand {
    /// List all tracked applications
    List,

    /// Show detailed info of an application
    Detail {
        /// Application ID
        app_id: usize,
    },

    /// Get the category of an application
    Category {
        /// Application ID
        app_id: usize,
    },

    /// Set category for one or more applications
    SetCategory {
        /// Category ID
        #[arg(long)]
        category: u64,

        /// Application IDs
        #[arg(required = true, num_args = 1..)]
        app_ids: Vec<usize>,
    },

    /// List applications not assigned to any category
    Uncategorized {
        /// Start offset for pagination
        #[arg(long, default_value_t = 0)]
        offset: usize,

        /// Maximum number of applications to return
        #[arg(long, default_value_t = 20)]
        limit: usize,

        /// Filter by name, company or path
        #[arg(long)]
        keyword: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum CategoryCommand {
    /// Show category tree
    List,

    /// List apps in a category
    Apps {
        /// Category ID
        id: u64,
    },

    /// Add a new category
    Add {
        /// Category name
        name: String,

        /// Parent category ID (default: root)
        #[arg(long, default_value_t = crate::state::category::ROOT_NODE_ID)]
        parent: u64,
    },

    /// Rename a category
    Rename {
        /// Category ID
        id: u64,

        /// New category name
        name: String,
    },

    /// Delete a category and unassign its apps
    Delete {
        /// Category ID
        id: u64,
    },
}

/// Run one-shot CLI commands. Should be called from an async runtime.
pub async fn run(cli: Cli) {
    if let Err(e) = engine_start(data_dir(), EngineMode::ReadOnly, |_| None) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }

    let Some(command) = cli.command else {
        eprintln!("No command specified. Run '{} --help' for usage.", APP_NAME);
        std::process::exit(2);
    };

    let json = cli.json;
    let result = match command {
        Commands::Time => time(json),
        Commands::App { command } => app::handle(command, json).await,
        Commands::Usage(args) => app::usage(args, json).await,
        Commands::Records(args) => records(args, json),
        Commands::Category { command } => category::handle(command, json).await,
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn time(json: bool) -> Result<(), String> {
    let now = chrono::Local::now();
    if json {
        let value = serde_json::json!({
            "local": now.to_rfc3339(),
            "utc": now.with_timezone(&chrono::Utc).to_rfc3339(),
            "unixMs": now.timestamp_millis(),
        });
        return print_json(&value);
    }
    println!("{}", now.to_rfc3339());
    Ok(())
}

pub(crate) fn now_timestamp() -> Timestamp {
    chrono::Local::now().timestamp_millis()
}

/// Parse a user provided time string: unix milliseconds, ISO-8601 with offset,
/// 'YYYY-MM-DD HH:MM:SS' or 'YYYY-MM-DD' (interpreted as local time).
pub(crate) fn parse_timestamp(s: &str) -> Result<Timestamp, String> {
    use chrono::TimeZone;

    if let Ok(ms) = s.parse::<Timestamp>() {
        return Ok(ms);
    }
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(s) {
        return Ok(dt.timestamp_millis());
    }
    for fmt in ["%Y-%m-%d %H:%M:%S", "%Y-%m-%d"] {
        let naive = chrono::NaiveDateTime::parse_from_str(s, fmt).or_else(|_| {
            chrono::NaiveDate::parse_from_str(s, fmt).map(|d| d.and_hms_opt(0, 0, 0).unwrap())
        });
        if let Ok(naive) = naive {
            if let Some(dt) = chrono::Local.from_local_datetime(&naive).earliest() {
                return Ok(dt.timestamp_millis());
            }
        }
    }
    Err(format!(
        "Invalid time '{s}'. Expected ISO-8601 (2026-08-01T00:00:00+08:00), \
         date (2026-08-01), 'YYYY-MM-DD HH:MM:SS' or unix milliseconds."
    ))
}

fn records(args: RecordsArgs, json: bool) -> Result<(), String> {
    let start_ts = args
        .start
        .as_deref()
        .map(parse_timestamp)
        .transpose()?
        .unwrap_or(0);
    let end_ts = args
        .end
        .as_deref()
        .map(parse_timestamp)
        .transpose()?
        .unwrap_or_else(now_timestamp);
    if start_ts >= end_ts {
        return Err("Start time must be before end time".to_string());
    }

    let mut records = read_by_timestamp(start_ts, end_ts);
    if let Some(app_id) = args.app_id {
        records.retain(|r| r.id == app_id);
    }
    if let Some(limit) = args.limit {
        records.truncate(limit);
    }

    if json {
        return print_json(&records);
    }

    let rows: Vec<Vec<String>> = records
        .iter()
        .map(|r| {
            vec![
                r.id.to_string(),
                format_timestamp(r.focus_at),
                format_timestamp(r.blur_at),
                format_duration(r.duration()),
            ]
        })
        .collect();
    print_table(&["ID", "Start", "End", "Duration"], &rows);
    println!("Total records: {}", records.len());
    Ok(())
}
