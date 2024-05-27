use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    process::exit,
};

use windows::{
    core::HSTRING,
    Win32::{
        Foundation::{GetLastError, BOOL, ERROR_ALREADY_EXISTS, HWND, LPARAM},
        System::Threading::{CreateMutexW, GetCurrentProcessId},
        UI::WindowsAndMessaging::{
            EnumWindows, GetWindowThreadProcessId, SetForegroundWindow, ShowWindow, SW_SHOW,
        },
    },
};

use super::global;
/// Check if the application has one instance which is already started.
/// If has, then stop this application.
pub fn check_singleton() {
    let mut mutex = unsafe { CreateMutexW(None, true, &HSTRING::from("TmusSingletonLock")) };
    let last_error = unsafe { GetLastError() };

    if last_error == ERROR_ALREADY_EXISTS {
        println!("Another instance is already running.");
        let pid = read_lock_pid();
        focus_already_started_instance_window(pid as isize);
        std::process::exit(0);
    }

    if mutex.is_err() {
        eprintln!("Failed to create mutex, {}", mutex.err().unwrap());
        exit(1);
    }

    let cur_pid = unsafe { GetCurrentProcessId() };
    write_lock_pid(cur_pid);
}

fn lock_file() -> File {
    let path = global::DATA_DIR.get().unwrap().join("lock");
    OpenOptions::new()
        .write(true)
        .read(true)
        .open(path)
        .expect("Failed to create lock file.")
}

fn write_lock_pid(pid: u32) {
    lock_file().write(&pid.to_le_bytes()).unwrap();
}

fn read_lock_pid() -> u32 {
    let mut buf = [0; 4];
    lock_file().read(&mut buf).unwrap();
    u32::from_le_bytes(buf)
}

pub fn focus_already_started_instance_window(pid: isize) {
    unsafe {
        EnumWindows(Some(enum_windows_callback), LPARAM(pid)).unwrap();
    }
}

unsafe extern "system" fn enum_windows_callback(hwnd: HWND, l_param: LPARAM) -> BOOL {
    let mut pid = 0;
    GetWindowThreadProcessId(hwnd, Some(&mut pid));

    if pid == l_param.0 as u32 {
        ShowWindow(hwnd, SW_SHOW).unwrap();
        SetForegroundWindow(hwnd).unwrap();
        return BOOL(0);
    }
    BOOL(1)
}
