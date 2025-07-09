use super::{models::FocusEvent, monitor, tracking};
use crate::config::{INVALID_INTERVAL_BOUND, LOOP_GET_CURRENT_WINDOW_INTERVAL};
use crate::monitor::loop_get_current_window;
use crate::util::Timestamp;
use std::{
    path::PathBuf,
    sync::{
        OnceLock,
        mpsc::{Receiver, Sender, channel},
    },
    thread,
};

pub static FOCUS_EVENT_SENDER: OnceLock<Sender<FocusEvent>> = OnceLock::new();

/// Initializes the application engine with a given data directory and a filter function.
///
/// This function sets up the necessary communication channels, initializes the `AlphaEngine`,
/// and starts the monitoring process. It also sets the event hook for the monitor.
///
/// # Arguments
/// * `data_dir` - A reference to the `PathBuf` representing the data directory.
/// * `filter` - A function pointer that takes a `&str` and returns an `Option<String>`.
///              This function is used to filter the application paths before recording.
///
/// # Panics
/// This function will panic if the `FOCUS_EVENT_SENDER` or `ENGINE` cannot be set.
pub fn engine_init(data_dir: &PathBuf) -> Receiver<FocusEvent> {
    let (sender, receiver) = channel::<FocusEvent>();
    FOCUS_EVENT_SENDER.set(sender).unwrap();
    tracking::init(data_dir);
    receiver
}

pub struct FocusRecordRaw {
    pub app_path: String,
    pub focus_at: Timestamp,
    pub blur_at: Timestamp,
}

impl FocusRecordRaw {
    fn new(app_path: String, focus_at: Timestamp, blur_at: Timestamp) -> Self {
        Self {
            app_path,
            focus_at,
            blur_at,
        }
    }
}

pub fn engine_start(filter: fn(&str) -> Option<String>, receiver: Receiver<FocusEvent>) {
    let write_record = move |raw: FocusRecordRaw| {
        if let Some(app_path) = filter(&raw.app_path) {
            tracking::write_record(FocusRecordRaw::new(app_path, raw.focus_at, raw.blur_at))
        } else {
            log::info!(
                "App {} is filtered out. Focus at {:?}, blur at {:?}",
                raw.app_path,
                raw.focus_at,
                raw.blur_at
            )
        }
    };

    thread::spawn(move || {
        let mut last_receive = 0;
        let mut last_focus = FocusEvent {
            app_path: String::default(),
            focus_at: Timestamp::MAX,
        };
        loop {
            let cur_focus = receiver.recv().unwrap();
            log::info!(
                "Receive focus event, last: {:?}, current: {:?}",
                last_focus,
                cur_focus
            );

            if cur_focus.focus_at - last_receive > INVALID_INTERVAL_BOUND {
                write_record(FocusRecordRaw::new(
                    last_focus.app_path,
                    last_focus.focus_at,
                    last_receive,
                ));
                last_receive = cur_focus.focus_at;
                last_focus = cur_focus;
                log::info!(
                    "New window focus event timeout. Last receive at {:?}",
                    last_receive
                );
                continue;
            }
            last_receive = cur_focus.focus_at;

            if cur_focus.app_path == last_focus.app_path {
                continue;
            }

            write_record(FocusRecordRaw::new(
                last_focus.app_path,
                last_focus.focus_at,
                cur_focus.focus_at,
            ));
            last_focus = cur_focus;
        }
    });
    thread::spawn(move || loop_get_current_window(LOOP_GET_CURRENT_WINDOW_INTERVAL));
    monitor::set_event_hook();
}
