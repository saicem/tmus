use clap::{Parser, Subcommand, ValueEnum};
use tracing::Level;

use crate::app::constant::APP_NAME;
use crate::cmd::read_by_timestamp;
use tmus_engine::storage::focus_app;
use tmus_engine::util::Timestamp;

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
#[command(
    name = APP_NAME,
    version,
    about = "Application usage tracker"
)]
pub struct Cli {
    #[arg(
        long,
        value_name = "LEVEL",
        help = "Set log level (trace, debug, info, warn, error)"
    )]
    pub level: Option<LogLevel>,

    #[arg(long, help = "Start without window (daemon mode)")]
    pub nw: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "List all tracked applications")]
    App {
        #[arg(short, long, help = "Show app IDs")]
        id: bool,
    },
    #[command(about = "Query focus records")]
    Records {
        #[arg(short, long, help = "Start timestamp (ms)")]
        start: Option<Timestamp>,
        #[arg(short, long, help = "End timestamp (ms)")]
        end: Option<Timestamp>,
        #[arg(short, long, help = "App ID to filter")]
        app_id: Option<usize>,
        #[arg(short, long, help = "Limit number of records")]
        limit: Option<usize>,
    },
}

pub fn handle_cli(cli: Cli) {
    let l: Level = cli.level.unwrap_or(LogLevel::Info).into();
    tracing::subscriber::with_default(tracing_subscriber::fmt().with_max_level(l).finish(), || {});

    match cli.command {
        Some(Commands::App { id }) => {
            list_apps(id);
        }
        Some(Commands::Records {
            start,
            end,
            app_id,
            limit,
        }) => {
            query_records(start, end, app_id, limit);
        }
        None => {}
    }
}

fn list_apps(show_id: bool) {
    let apps = focus_app::get_all_app();
    for (i, app) in apps.iter().enumerate() {
        if show_id {
            println!("{}: {}", i, app);
        } else {
            println!("{}", app);
        }
    }
}

fn query_records(
    start: Option<Timestamp>,
    end: Option<Timestamp>,
    app_id: Option<usize>,
    limit: Option<usize>,
) {
    let start_ts = start.unwrap_or(0);
    let end_ts = end.unwrap_or_else(|| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as Timestamp
    });

    let records = read_by_timestamp(start_ts, end_ts);

    let filtered: Vec<_> = if let Some(id) = app_id {
        records.into_iter().filter(|r| r.id == id).collect()
    } else {
        records
    };

    let limited: Vec<_> = if let Some(l) = limit {
        filtered.into_iter().take(l).collect()
    } else {
        filtered
    };

    let count = limited.len();
    for record in limited {
        println!("{:?}", record);
    }

    println!("Total records: {}", count);
}
