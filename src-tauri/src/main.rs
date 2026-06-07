// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use tmus::{init_tracing, run_cli_mode, run_gui_mode, Cli};

fn main() {
    let cli = Cli::parse();

    init_tracing(cli.level);

    if cli.command.is_some() {
        run_cli_mode(cli);
        return;
    }

    run_gui_mode(cli);
}
