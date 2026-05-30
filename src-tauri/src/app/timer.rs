use tokio::time::{interval, Duration};
use tracing::{error, info};

use crate::state::{category::save_category_data, scheme::save_statistic_scheme};

// Start a timer to save data
pub fn start_timer() {
    info!("Starting save timer");
    tauri::async_runtime::spawn(async move {
        let mut interval = interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            save_data();
        }
    });
}

fn save_data() {
    if let Err(e) = save_category_data() {
        error!("Failed to save category data: {}", e);
    }
    if let Err(e) = save_statistic_scheme() {
        error!("Failed to save statistic scheme: {}", e);
    }
}
