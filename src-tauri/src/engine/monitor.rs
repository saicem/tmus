use log;
use std::time::Duration;
use tokio::time;
use windows::core::Result;
use windows::core::PWSTR;
use windows::Win32::Foundation::*;
use windows::Win32::System::Threading::*;
use windows::Win32::UI::Accessibility::SetWinEventHook;
use windows::Win32::UI::Accessibility::HWINEVENTHOOK;
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::engine::core::FOCUS_EVENT_SENDER;
use crate::engine::models::FocusEvent;
use crate::engine::models::Millisecond;

pub fn loop_get_current_window(interval: Duration) {
    tauri::async_runtime::spawn(async move {
        let mut interval = time::interval(interval);
        loop {
            interval.tick().await;
            let hwnd = unsafe { GetForegroundWindow() };
            on_window_focus(&hwnd);
        }
    });
}

pub fn set_event_hook() {
    log::info!("Set foreground change event hook");
    unsafe {
        SetWinEventHook(
            EVENT_SYSTEM_FOREGROUND,
            EVENT_SYSTEM_FOREGROUND,
            None,
            Some(on_foreground_changed),
            0,
            0,
            WINEVENT_OUTOFCONTEXT,
        )
    };
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn on_foreground_changed(
    _: HWINEVENTHOOK,
    _: u32,
    hwnd: HWND,
    _: i32,
    _: i32,
    _: u32,
    _: u32,
) {
    on_window_focus(&hwnd);
}

fn on_window_focus(hwnd: &HWND) {
    match get_process_path_from_hwnd(&hwnd) {
        Ok(process_path) => {
            log::info!("On window focus: {}", &process_path);
            FOCUS_EVENT_SENDER
                .get()
                .expect("Not init focus event sender.")
                .send(FocusEvent {
                    app_path: process_path,
                    focus_at: Millisecond::now(),
                })
                .expect("[Monitor] Failed to send focus event.");
        }
        Err(e) => {
            log::error!("Error, can't get process path: {}", e);
        }
    }
}

/// Get the path of the process.
/// Here are some probabilities of failure:
/// - The process just exited, couldn't open the process.
pub fn get_process_path_from_hwnd(hwnd: &HWND) -> Result<String> {
    let mut text: [u16; 1024] = [0; 1024];
    let mut process_name_length: u32 = 1024;
    let mut process_id: u32 = 0;

    unsafe {
        GetWindowThreadProcessId(*hwnd, Some(&mut process_id));
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, true, process_id)?;
        QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_WIN32,
            PWSTR(text.as_mut_ptr()),
            &mut process_name_length,
        )?;
    }
    Ok(String::from_utf16_lossy(
        &text[..process_name_length as usize],
    ))
}
