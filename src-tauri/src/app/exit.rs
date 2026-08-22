use crate::state::category::save_category_data;

pub fn app_exit() {
    save_on_exit();
    std::process::exit(0);
}

pub fn save_on_exit() {
    let _ = save_category_data();
}
