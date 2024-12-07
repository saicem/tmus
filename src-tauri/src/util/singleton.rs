use log::{error, info};
use std::process::exit;
use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{GetLastError, ERROR_ALREADY_EXISTS},
        System::Threading::{CreateMutexW, GetCurrentProcessId},
    },
};

/// Check if the application has one instance which is already started.
/// If it has, then stop this application.
pub fn force_singleton(key: &str) {
    let mutex = unsafe { CreateMutexW(None, true, &HSTRING::from(key)) };
    let last_error = unsafe { GetLastError() };

    if last_error == ERROR_ALREADY_EXISTS {
        info!("Another instance is already running.");
        exit(0);
    }

    if mutex.is_err() {
        error!("Failed to create mutex, {}", mutex.err().unwrap());
        exit(1);
    }

    let cur_pid = unsafe { GetCurrentProcessId() };
}
