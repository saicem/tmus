use super::{
    alpha::engine::AlphaEngine,
    data::{AppId, EngineError, FocusEvent, Millisecond},
    monitor, FocusRecord,
};
use crate::engine::monitor::loop_get_current_window;
use std::{
    path::PathBuf,
    sync::{
        mpsc::{channel, Receiver, Sender},
        OnceLock,
    },
    thread,
    time::Duration,
};

pub static FOCUS_EVENT_SENDER: OnceLock<Sender<FocusEvent>> = OnceLock::new();

pub trait Engine {
    fn new(data_dir: &PathBuf) -> Self;
    fn read_by_time(&self, start: Millisecond, end: Millisecond) -> Vec<FocusRecord>;
    fn get_id_by_path(&self, path: &str) -> AppId;
    fn get_path_by_id(&self, id: AppId) -> Result<String, EngineError>;
    fn write_record(&self, raw: FocusRecordRaw);
}

static ENGINE: OnceLock<AlphaEngine> = OnceLock::new();

pub fn get_engine<'a>() -> &'a impl Engine {
    ENGINE.get().unwrap()
}

pub fn init(data_dir: &PathBuf) {
    let (sender, receiver) = channel::<FocusEvent>();
    FOCUS_EVENT_SENDER.set(sender).unwrap();
    let alpha_engine = AlphaEngine::new(data_dir);
    start(|raw| ENGINE.get().unwrap().write_record(raw), receiver);
    ENGINE.set(alpha_engine).expect("Engine init failed.");
    monitor::set_event_hook();
}

pub struct FocusRecordRaw<'a> {
    pub app_path: &'a str,
    pub focus_at: Millisecond,
    pub blur_at: Millisecond,
}

impl<'a> FocusRecordRaw<'a> {
    fn new(app_path: &'a str, focus_at: Millisecond, blur_at: Millisecond) -> Self {
        Self {
            app_path,
            focus_at,
            blur_at,
        }
    }
}

fn start<F>(write_record: F, receiver: Receiver<FocusEvent>)
where
    F: Fn(FocusRecordRaw) + Send + 'static,
{
    /// Check the current window every 1 minute
    static LOOP_GET_CURRENT_WINDOW_INTERVAL: Duration = Duration::from_secs(1 * 60);
    /// If foreground change event interval above this threshold, it's invalid.
    static INVALID_INTERVAL_BOUND: Millisecond = Millisecond::from_secs(3 * 60);
    thread::spawn(move || {
        let mut last_receive = Millisecond::ZERO;
        let mut last_focus = FocusEvent {
            app_path: String::default(),
            focus_at: Millisecond::MAX,
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
                    &last_focus.app_path,
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
                &last_focus.app_path,
                last_focus.focus_at,
                cur_focus.focus_at,
            ));
            last_focus = cur_focus;
        }
    });
    loop_get_current_window(LOOP_GET_CURRENT_WINDOW_INTERVAL);
}

fn on_foreground_changed(process_path: &str) {
    let sender = FOCUS_EVENT_SENDER.get().unwrap();
    sender
        .send(FocusEvent {
            app_path: process_path.to_string(),
            focus_at: Millisecond::now(),
        })
        .unwrap();
}
