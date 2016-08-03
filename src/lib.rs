//! nanosleep for rust
//!
//!
//! # Goals
//! * provide nanosleep for linux and macos
//!
//! # Future work
//! * other OS support?
//!
//! # Usage
//!
//! Include the module and call ```shuteye::sleep(Duration::new(seconds, nanoseconds))```
//!
//! # Example
//!
//! Sleep for 1 uS
//!
//! ```
//! use shuteye::sleep;
//! use std::time::Duration;
//!
//! // call sleep
//! sleep(Duration::new(1, 0));

#![crate_type = "lib"]

#![crate_name = "shuteye"]

use std::time::Duration;

const TIMER_RELTIME: i32 = 0;
const CLOCK_MONOTONIC: i32 = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
struct Timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

/// sleep for a relative time
///
/// # Example
/// ```
/// use std::time::Duration;
/// use shuteye::sleep;
///
/// // simple sleep for 1s duration
/// sleep(Duration::new(1, 0));
///
/// // remain captures remaining time from `Timespec`
/// match sleep(Duration::new(1, 0)) {
///     Some(remain) => {
///         // woke early - some sleep time remains
///     }
///     None => {
///         // woke on-time or late - no sleep time remains
///     }
/// }
pub fn sleep(duration: Duration) -> Option<Duration> {
    let ts = duration_to_timespec(duration);
    let mut remain = Timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_nanosleep(CLOCK_MONOTONIC, TIMER_RELTIME, &ts, Some(&mut remain));
    if remain.tv_nsec == 0 && remain.tv_sec == 0 {
        return None;
    }
    Some(timespec_to_duration(remain))
}

fn duration_to_timespec(duration: Duration) -> Timespec {
    Timespec {
        tv_sec: duration.as_secs() as i64,
        tv_nsec: duration.subsec_nanos() as i64,
    }
}

fn timespec_to_duration(timespec: Timespec) -> Duration {
    Duration::new(timespec.tv_sec as u64, timespec.tv_nsec as u32)
}

#[cfg(target_os = "linux")]
fn clock_nanosleep(id: i32, flags: i32, req: &Timespec, remain: Option<&mut Timespec>) -> i32 {
    extern "C" {
        fn clock_nanosleep(clock_id: i32,
                           flags: i32,
                           req: *const Timespec,
                           rem: *mut Timespec)
                           -> i32;
    }
    match remain {
        Some(p) => unsafe { clock_nanosleep(id, flags, req as *const _, p as *mut _) },
        _ => unsafe { clock_nanosleep(id, flags, req as *const _, 0 as *mut _) },
    }
}

#[cfg(target_os = "macos")]
fn clock_nanosleep(_: i32, _: i32, req: &Timespec, remain: Option<&mut Timespec>) -> i32 {
    extern "C" {
        fn nanosleep(req: *const Timespec, rem: *mut Timespec) -> i32;
    }
    match remain {
        Some(p) => unsafe { nanosleep(req as *const _, p as *mut _) },
        _ => unsafe { nanosleep(req as *const _, 0 as *mut _) },
    }
}
