use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use crate::app::data::TmusTick;
use once_cell::sync::Lazy;
use windows::core::PWSTR;
use windows::Win32::Foundation::*;
use windows::Win32::System::Threading::*;
use windows::Win32::UI::Accessibility::SetWinEventHook;
use windows::Win32::UI::Accessibility::HWINEVENTHOOK;
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::app::persist::{get_id_by_name, write_record};

static CHANNEL_FOCUS: Lazy<Arc<(Sender<String>, Mutex<Receiver<String>>)>> = Lazy::new(|| {
    Arc::new({
        let (tx, rx) = mpsc::channel::<String>();
        (tx, Mutex::new(rx))
    })
});

pub fn set_event_hook() {
    println!("Set event hook");
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

    thread::spawn(move || {
        let receiver = &CHANNEL_FOCUS.1.lock().unwrap();
        let mut last_process = foreground_process_path();
        let mut last_focus = TmusTick::now();
        loop {
            let cur_process = receiver.recv().unwrap();
            println!("recv: {}", cur_process.clone());
            if cur_process == last_process {
                continue;
            }

            let cur_focus = TmusTick::now();
            let app_id = get_id_by_name(&last_process);
            write_record(app_id, last_focus, cur_focus.clone());

            last_focus = cur_focus;
            last_process = cur_process;
        }
    });
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
    let process_path = process_path(&hwnd);

    println!("foreground change: {}", process_path.clone());

    CHANNEL_FOCUS
        .0
        .send(process_path)
        .expect("send channel message failed.");
}

pub fn process_path(hwnd: &HWND) -> String {
    let mut text: [u16; 1024] = [0; 1024];
    let mut process_name_length: u32 = 1024;
    let mut process_id: u32 = 0;

    unsafe {
        GetWindowThreadProcessId(*hwnd, Some(&mut process_id));
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, true, process_id)
            .expect("win32 OpenProcess failed.");
        QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_WIN32,
            PWSTR(text.as_mut_ptr()),
            &mut process_name_length,
        )
        .expect("win32 QueryFullProcessImageNameW failed.")
    }
    String::from_utf16_lossy(&text[..process_name_length as usize])
}

pub fn foreground_process_path() -> String {
    unsafe { process_path(&GetForegroundWindow()) }
}
