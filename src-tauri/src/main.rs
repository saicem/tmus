// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use tmus::{init_tracing, run_cli_mode, run_gui_mode, Cli, LogLevel};

fn main() {
    let cli = Cli::parse();

    // CLI output should stay clean by default; --level can lower it for debugging.
    let default_level = if cli.command.is_some() {
        LogLevel::Warn
    } else {
        LogLevel::Info
    };
    init_tracing(cli.level.unwrap_or(default_level));

    if cli.command.is_some() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to build CLI runtime");
        runtime.block_on(run_cli_mode(cli));
        return;
    }

    run_gui_mode(cli);
}
