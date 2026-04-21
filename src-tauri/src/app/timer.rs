use tracing::info;

use crate::state::{category::save_category_data, scheme::save_statistic_scheme};

// Start a timer to save data
pub fn start_timer() {
    info!("Starting save timer");
    std::thread::spawn(|| loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
        save_category_data();
        save_statistic_scheme();
    });
}
