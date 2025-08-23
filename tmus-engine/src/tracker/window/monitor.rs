use crate::async_runtime::handle;
use crate::tracker::window::{WindowFocusEvent, WindowTrackerConfig};
use crate::util::{Timestamp, now_timestamp};
use std::sync::OnceLock;
use std::time::Duration;
use tokio::sync::mpsc::Sender;
use tokio::time;
use tracing::{debug, error, info};
use windows::Win32::Foundation::*;
use windows::Win32::System::Threading::*;
use windows::Win32::UI::Accessibility::HWINEVENTHOOK;
use windows::Win32::UI::Accessibility::SetWinEventHook;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::core::PWSTR;
use windows::core::Result;

static FOCUS_EVENT_SENDER: OnceLock<Sender<WindowFocusEvent>> = OnceLock::new();

pub fn set_window_tracker(sender: Sender<WindowFocusEvent>) {
    let config = WindowTrackerConfig::default();
    FOCUS_EVENT_SENDER.set(sender).unwrap();
    set_event_hook();
    handle().spawn(loop_get_current_window(
        config.loop_get_current_window_interval,
    ));
}

async fn loop_get_current_window(interval: Duration) {
    let mut interval = time::interval(interval);
    loop {
        interval.tick().await;
        let process_path = get_process_path_from_hwnd(&unsafe { GetForegroundWindow() })
            .expect("Failed to get process path");
        send_focus_event(process_path, now_timestamp()).await;
    }
}

fn set_event_hook() {
    info!("Set foreground change event hook");
    unsafe {
        let hook = SetWinEventHook(
            EVENT_SYSTEM_FOREGROUND,
            EVENT_SYSTEM_FOREGROUND,
            None,
            Some(on_foreground_changed),
            0,
            0,
            WINEVENT_OUTOFCONTEXT,
        );
        if hook.is_invalid() {
            error!("Failed to set foreground change event hook");
            return;
        }
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
    debug!("On foreground changed {:?}", hwnd);
    let process_path = get_process_path_from_hwnd(&hwnd);
    if let Ok(process_path) = process_path {
        handle().spawn(send_focus_event(process_path, now_timestamp()));
    } else {
        debug!("Failed to get process path: {:?}", process_path);
        return;
    }
}

async fn send_focus_event(process_path: String, focus_at: Timestamp) {
    debug!("On window focus: {}", &process_path);
    FOCUS_EVENT_SENDER
        .get()
        .expect("Not init focus event sender.")
        .send(WindowFocusEvent {
            app_path: process_path,
            focus_at,
        })
        .await
        .expect("[Monitor] Failed to send focus event.")
}

/// Get the path of the process.
/// Here are some probabilities of failure:
/// - The process just exited, couldn't open the process.
fn get_process_path_from_hwnd(hwnd: &HWND) -> Result<String> {
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
