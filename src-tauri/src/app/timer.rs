use tracing::info;

use crate::state::category::save_category_data;

// Start a timer to check and save category data every minute
pub fn start_category_timer() {
    info!("Starting category timer");
    std::thread::spawn(|| loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
        save_category_data();
    });
}
