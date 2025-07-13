use crate::async_runtime::handle;
use crate::tracker::window::{WindowFocusEvent, set_window_tracker};
use crate::util::{Timestamp, m_as_ms};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tracing::debug;

pub struct TrackingSpanConfig {
    /// If foreground change event interval above this threshold, it's invalid.
    invalid_interval_bound: Timestamp,
}

impl Default for TrackingSpanConfig {
    fn default() -> Self {
        Self {
            invalid_interval_bound: m_as_ms(3),
        }
    }
}

#[derive(Debug)]
pub struct TrackingSpanEvent {
    pub name: String,
    pub focus_at: Timestamp,
    pub blur_at: Timestamp,
}

pub fn start_tracking<F>(on_span_event: F)
where
    F: Fn(TrackingSpanEvent) + Send + Sync + 'static,
{
    let config = TrackingSpanConfig::default();
    let (window_sender, window_receiver) = mpsc::channel::<WindowFocusEvent>(16);
    set_window_tracker(window_sender);
    handle().spawn(handle_window_event(on_span_event, config, window_receiver));
}

pub async fn handle_window_event<F>(
    on_span_event: F,
    config: TrackingSpanConfig,
    mut window_receiver: Receiver<WindowFocusEvent>,
) where
    F: Fn(TrackingSpanEvent),
{
    let mut span_last_recv_at = 0;
    let mut span_first_recv = WindowFocusEvent {
        app_path: String::default(),
        focus_at: Timestamp::MAX,
    };
    loop {
        let this_recv = window_receiver
            .recv()
            .await
            .expect("[TrackingSpan] Failed to receive window focus event.");
        debug!("On window focus event recv: {:?}", &this_recv);

        // Consider computer is sleep.
        if this_recv.focus_at - span_last_recv_at > config.invalid_interval_bound {
            debug!("[TrackingSpan] Invalid interval bound.");
            on_span_event(TrackingSpanEvent {
                name: span_first_recv.app_path.to_string(),
                focus_at: span_first_recv.focus_at,
                blur_at: span_last_recv_at,
            });
            span_last_recv_at = this_recv.focus_at;
            span_first_recv = this_recv;
            continue;
        }

        // Same app merge to one record.
        if this_recv.app_path == span_first_recv.app_path {
            span_last_recv_at = this_recv.focus_at;
            continue;
        }

        // App change, emit span.
        on_span_event(TrackingSpanEvent {
            name: span_first_recv.app_path.to_string(),
            focus_at: span_first_recv.focus_at,
            blur_at: this_recv.focus_at,
        });
        span_last_recv_at = this_recv.focus_at;
        span_first_recv = this_recv;
    }
}
