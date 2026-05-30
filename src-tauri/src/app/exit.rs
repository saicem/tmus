use crate::state::{category::save_category_data, scheme::save_statistic_scheme};

pub fn app_exit() {
    save_on_exit();
    std::process::exit(0);
}

pub fn save_on_exit() {
    let _ = save_category_data();
    let _ = save_statistic_scheme();
}
