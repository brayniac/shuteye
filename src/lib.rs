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
//! Include the module, create a timespec, call clock_nanosleep
//!
//! # Example
//!
//! Sleep for 1 uS
//!
//! ```
//! use shuteye::*;
//!
//! // create a timespec for the interval to sleep
//! let ts = Timespec::from_nano(1000).unwrap();
//! assert_eq!(ts.get_sec(), 0);
//! assert_eq!(ts.get_nsec(), 1000);
//!
//! // call sleep
//! shuteye::sleep(ts);

#![crate_type = "lib"]

#![crate_name = "shuteye"]

use std::fmt;

const TIMER_RELTIME: i32 = 0;
const NSEC_PER_SEC: i64 = 1_000_000_000;
const CLOCK_MONOTONIC: i32 = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

impl Timespec {

    /// create a Timespec from nanoseconds
    ///
    /// # Example
    /// ```
    /// use shuteye::*;
    ///
    /// assert!(Timespec::from_nano(1000).is_ok());
    pub fn from_nano(nsec: i64) -> Result<Timespec, &'static str> {
        if nsec < 0 {
            return Err("not implemented");
        }
        if nsec >= NSEC_PER_SEC {
            let sec = (nsec as f64 / NSEC_PER_SEC as f64).floor() as i64;
            let nsec = nsec - (sec * NSEC_PER_SEC);
            return Ok(Timespec {
                tv_sec: sec,
                tv_nsec: nsec,
            });
        }
        return Ok(Timespec {
            tv_sec: 0,
            tv_nsec: nsec,
        });
    }

    /// return seconds component of Timespec
    ///
    /// # Example
    /// ```
    /// use shuteye::*;
    ///
    /// let ts = Timespec::from_nano(1000000000).unwrap();
    ///
    /// assert_eq!(ts.get_sec(), 1);
    pub fn get_sec(self) -> i64 {
        self.tv_sec
    }

    /// return nanoseconds component of Timespec
    ///
    /// # Example
    /// ```
    /// use shuteye::*;
    ///
    /// let ts = Timespec::from_nano(1).unwrap();
    ///
    /// assert_eq!(ts.get_nsec(), 1);
    pub fn get_nsec(self) -> i64 {
        self.tv_nsec
    }

    /// return Timespec as nanoseconds
    ///
    /// # Example
    /// ```
    /// use shuteye::*;
    ///
    /// let ts = Timespec::from_nano(1000000001).unwrap();
    ///
    /// assert_eq!(ts.as_nsec(), 1000000001);
    pub fn as_nsec(self) -> i64 {
        self.tv_sec * NSEC_PER_SEC + self.tv_nsec
    }
}

impl fmt::Debug for Timespec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "timespec(sec: {}, nsec: {})", self.tv_sec, self.tv_nsec)
    }
}

/// sleep for a relative time
///
/// # Example
/// ```
/// use shuteye::*;
///
/// let ts = Timespec::from_nano(1000).unwrap();
///
/// // simple sleep
/// shuteye::sleep(ts);
///
/// // remain captures remaining time from `Timespec`
/// match shuteye::sleep(ts) {
///     Some(remain) => {
///         // some sleep time remains
///     }
///     None => {
///         // no sleep time remains
///     }
/// }
pub fn sleep(ts: Timespec) -> Option<Timespec> {
    let mut remain = Timespec::from_nano(0_i64).unwrap();
    clock_nanosleep(CLOCK_MONOTONIC, TIMER_RELTIME, &ts, Some(&mut remain));
    if remain.get_nsec() == 0 && remain.get_sec() == 0 {
        return None;
    }
    Some(remain)
}

#[cfg(target_os = "linux")]
fn clock_nanosleep(id: i32, flags: i32, req: &Timespec, remain: Option<&mut Timespec>) -> i32 {
    extern {
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
    extern {
        fn nanosleep(req: *const Timespec, rem: *mut Timespec) -> i32;
    }
    match remain {
        Some(p) => unsafe { nanosleep(req as *const _, p as *mut _) },
        _ => unsafe { nanosleep(req as *const _, 0 as *mut _) },
    }
}
