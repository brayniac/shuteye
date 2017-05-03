extern crate shuteye;
extern crate time;


use shuteye::sleep;
use std::time::Duration;
use time::precise_time_ns;

fn duration_from_ns(duration_ns: u64) -> Duration {
    Duration::new(duration_ns / 1_000_000_000,
                  (duration_ns % 1_000_000_000) as u32)
}

fn measure_sleep(sleep_duration: Duration) -> Duration {
    let start = precise_time_ns();
    let _remain = sleep(sleep_duration);
    let end = precise_time_ns();

    let remain = match _remain {
        Some(remain) => remain,
        None => Duration::new(0, 0),
    };

    duration_from_ns(end - start) + remain
}

#[test]
fn sleep_1s() {
    let duration = Duration::new(1, 0);
    assert!(measure_sleep(duration) >= duration);
}

#[test]
fn sleep_less_1s() {
    let duration = Duration::new(0, 999_999_999);
    assert!(measure_sleep(duration) >= duration);
}
