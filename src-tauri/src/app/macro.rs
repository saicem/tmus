#[macro_export]
macro_rules! upk {
    ($cell:expr) => {
        $cell
            .get()
            .expect("The object has not been set yet.")
            .lock()
            .unwrap()
    };
}
