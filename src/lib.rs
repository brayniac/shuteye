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
//! sleep(Duration::new(0, 1000));

#![deny(warnings)]

extern crate libc;

use std::ptr;
use std::time::Duration;

/// sleep for a relative time
///
/// # Example
/// ```rust
/// use std::time::Duration;
/// use shuteye::sleep;
///
/// // simple sleep for 1s duration
/// sleep(Duration::new(1, 0));
///
/// // remain captures remaining time
/// match sleep(Duration::new(1, 0)) {
///     Some(remain) => {
///         // woke early - some sleep time remains
///     }
///     None => {
///         // woke on-time or late - no sleep time remains
///     }
/// }
/// ```
pub fn sleep(duration: Duration) -> Option<Duration> {
    let ts = duration_to_timespec(duration);
    let mut remain = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };

    #[cfg(target_os = "linux")]
    clock_nanosleep(libc::CLOCK_MONOTONIC, 0, &ts, Some(&mut remain));

    #[cfg(target_os = "macos")]
    nanosleep(&ts, Some(&mut remain));


    if remain.tv_nsec == 0 && remain.tv_sec == 0 {
        return None;
    }
    Some(timespec_to_duration(remain))
}

fn duration_to_timespec(duration: Duration) -> libc::timespec {
    libc::timespec {
        tv_sec: duration.as_secs() as libc::time_t,
        tv_nsec: duration.subsec_nanos() as libc::c_long,
    }
}

fn timespec_to_duration(timespec: libc::timespec) -> Duration {
    Duration::new(timespec.tv_sec as u64, timespec.tv_nsec as u32)
}

#[cfg(target_os = "linux")]
fn clock_nanosleep(clk_id: libc::clockid_t,
                   flags: libc::c_int,
                   req: &libc::timespec,
                   remain: Option<&mut libc::timespec>)
                   -> i32 {
    match remain {
        Some(p) => unsafe { libc::clock_nanosleep(clk_id, flags, req as *const _, p as *mut _) },
        _ => unsafe { libc::clock_nanosleep(clk_id, flags, req as *const _, ptr::null_mut()) },
    }
}

#[cfg(target_os = "macos")]
fn nanosleep(req: &libc::timespec, remain: Option<&mut libc::timespec>) -> i32 {
    match remain {
        Some(p) => unsafe { libc::nanosleep(req as *const _, p as *mut _) },
        _ => unsafe { libc::nanosleep(req as *const _, ptr::null_mut()) },
    }
}
