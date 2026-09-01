// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use tmus::{init_tracing, run_cli_mode, run_gui_mode, Cli, LogLevel};

/// Attach to the console of the launching terminal on Windows. Release builds
/// use the GUI subsystem, which does not receive a console by default, so CLI
/// output would otherwise be lost; re-point stdout/stderr at the console.
#[cfg(windows)]
fn attach_parent_console() {
    use std::ffi::c_void;

    extern "system" {
        fn AttachConsole(dw_process_id: u32) -> i32;
        fn GetStdHandle(n_std_handle: u32) -> *mut c_void;
        fn SetStdHandle(n_std_handle: u32, h_handle: *mut c_void) -> i32;
    }

    const ATTACH_PARENT_PROCESS: u32 = u32::MAX;
    const STD_OUTPUT_HANDLE: u32 = u32::MAX - 11;
    const STD_ERROR_HANDLE: u32 = u32::MAX - 12;

    unsafe {
        if AttachConsole(ATTACH_PARENT_PROCESS) == 0 {
            return;
        }
        // Write back whatever handle is now associated with the console; this
        // keeps pre-existing redirections (pipes) intact as well.
        let stdout = GetStdHandle(STD_OUTPUT_HANDLE);
        let stderr = GetStdHandle(STD_ERROR_HANDLE);
        if !stdout.is_null() {
            SetStdHandle(STD_OUTPUT_HANDLE, stdout);
        }
        if !stderr.is_null() {
            SetStdHandle(STD_ERROR_HANDLE, stderr);
        }
    }
}

fn main() {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(err) => {
            // --help/--version and parse errors are printed by clap before any
            // CLI mode is known, so attach the console here to make them
            // visible in release builds (GUI subsystem swallows output).
            #[cfg(windows)]
            attach_parent_console();
            let code = if err.use_stderr() { 2 } else { 0 };
            err.print().ok();
            std::process::exit(code);
        }
    };

    // Only real CLI invocations (a subcommand is present) need console output;
    // daemon mode (--nw) and plain GUI launches stay headless.
    #[cfg(windows)]
    if cli.command.is_some() {
        attach_parent_console();
    }

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
