use std::time::Instant;

pub fn log_time_used(start_time: Instant, info: &str) {
    let end_time = Instant::now();
    println!(
        "Info: {} time={:?}",
        info,
        end_time.duration_since(start_time)
    );
}
