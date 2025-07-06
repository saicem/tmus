use log::info;
use std::time::Instant;

pub fn compute_consume_time<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed().as_millis();
    info!("compute time: {}ms", duration);
    result
}
