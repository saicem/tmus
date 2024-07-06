use std::process::exit;
use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{GetLastError, ERROR_ALREADY_EXISTS},
        System::Threading::{CreateMutexW, GetCurrentProcessId},
    },
};

/// Check if the application has one instance which is already started.
/// If has, then stop this application.
pub fn force_singleton() {
    let mutex = unsafe { CreateMutexW(None, true, &HSTRING::from("TmusSingletonLock")) };
    let last_error = unsafe { GetLastError() };

    if last_error == ERROR_ALREADY_EXISTS {
        println!("Another instance is already running.");
        std::process::exit(0);
    }

    if mutex.is_err() {
        eprintln!("Failed to create mutex, {}", mutex.err().unwrap());
        exit(1);
    }

    let cur_pid = unsafe { GetCurrentProcessId() };
}
