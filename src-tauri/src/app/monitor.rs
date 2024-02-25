use std::{
    collections::HashMap,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
};

use crate::app::analyze::collect_record;
use chrono::{prelude::*, Duration};
use once_cell::sync::Lazy;
use windows::{
    core::PWSTR,
    Win32::{
        Foundation::*,
        System::Threading::*,
        UI::Accessibility::{SetWinEventHook, HWINEVENTHOOK},
        UI::WindowsAndMessaging::*,
    },
};

use crate::app::focus_log::FocusRecord;

use super::focus_log::FocusLogger;

static CHANNEL_FOCUS: Lazy<Arc<(Sender<String>, Mutex<Receiver<String>>)>> = Lazy::new(|| {
    Arc::new({
        let (tx, rx) = mpsc::channel::<String>();
        (tx, Mutex::new(rx))
    })
});
const DAY_CENTIS: u64 = (Duration::days(1).num_milliseconds() / 10) as u64;

pub fn set_event_hook(mut logger: FocusLogger) {
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
        let apps = logger.read_apps();
        let mut app_count: u64 = 0;
        let mut app_map = HashMap::new();

        // debug
        let records = logger.read_record();
        for (id, duration) in collect_record(records) {
            println!(
                "{} {} {}",
                id,
                apps[(id - 1) as usize].clone(),
                Duration::milliseconds((duration * 10) as i64)
            )
        }

        for app in apps {
            app_count += 1;
            println!("read: {} {}", app_count, app.clone());
            app_map.insert(app, app_count);
        }

        let receiver = &CHANNEL_FOCUS.1.lock().unwrap();

        let mut last_process = foreground_process_path();
        let mut last_focus = Utc::now();
        let last_focus_centis = (last_focus.timestamp_millis() / 10) as u64;
        let mut last_day = last_focus_centis / DAY_CENTIS;
        let mut last_centis = last_focus_centis - last_day * DAY_CENTIS;

        let mut index = logger.read_index();
        let log_start_day = if index.is_empty() {
            logger.write_index(last_day);
            last_day
        } else {
            index.remove(0)
        };
        let days = last_day - log_start_day - index.len() as u64;
        if days != 0 {
            let val = index.last().copied().unwrap_or(0);
            for _ in 0..days {
                index.push(val);
            }
        }

        loop {
            let cur_process = receiver.recv().unwrap();
            if cur_process == last_process {
                continue;
            }
            let app_id = app_map.entry(last_process.clone()).or_insert_with(|| {
                logger.write_app(&last_process);
                app_count += 1;
                app_count
            });

            let cur_focus = Utc::now();
            let cur_focus_centis = (cur_focus.timestamp_millis() / 10) as u64;
            let cur_day = cur_focus_centis / DAY_CENTIS;
            let cur_centis = cur_focus_centis - cur_day * DAY_CENTIS;

            for _ in cur_day..last_day {
                let duration_centis = DAY_CENTIS - last_centis;
                let len = logger.record_position();
                logger.write_index(len);
                index.push(len);
                logger.write_record(FocusRecord::new(
                    app_id.clone(),
                    last_centis,
                    duration_centis,
                ));
                last_centis = 0;
            }

            let duration = cur_focus - last_focus;

            let duration_centis = cur_centis - last_centis;

            logger.write_record(FocusRecord::new(
                app_id.clone(),
                last_centis,
                duration_centis,
            ));

            println!(
                "{} {} {} {} {}",
                app_id,
                last_focus.clone(),
                duration.clone(),
                last_process.clone(),
                cur_process.clone(),
            );

            last_day = cur_day;
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
