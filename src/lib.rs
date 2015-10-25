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
//! let ts = shuteye::timespec::from_nano(1000).unwrap();
//! assert_eq!(ts.get_sec(), 0);
//! assert_eq!(ts.get_nsec(), 1000);
//! shuteye::clock_nanosleep(1, 0, &ts, None);

#![crate_type = "lib"]

#![crate_name = "shuteye"]

use std::fmt;

const NSEC_PER_SEC: i64 = 1_000_000_000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

impl timespec {
    /// create a timespec from nanoseconds
    ///
    /// # Example
    /// ```
    /// use shuteye::*;
    ///
    /// assert!(shuteye::timespec::from_nano(1000).is_ok());
    pub fn from_nano(nsec: i64) -> Result<timespec, &'static str> {
        if nsec < 0 {
            return Err("not implemented");
        }
        if nsec > NSEC_PER_SEC {
            let sec = (nsec as f64 / NSEC_PER_SEC as f64).floor() as i64;
            let nsec = nsec - (sec * NSEC_PER_SEC);
            return Ok(timespec { tv_sec: sec, tv_nsec: nsec });
        }
        return Ok(timespec { tv_sec: 0, tv_nsec: nsec });
    }

    pub fn get_sec(self) -> i64 {
        self.tv_sec
    }

    pub fn get_nsec(self) -> i64 {
        self.tv_nsec
    }
}

impl fmt::Debug for timespec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "timespec(sec: {}, nsec: {})", self.tv_sec, self.tv_nsec)
    }
}

#[cfg(all(not(target_os = "macos"), not(target_os = "ios")))]
pub fn clock_nanosleep(id: i32, flags: i32, req: &timespec, remain: Option<&mut timespec>) -> i32 {
    extern {
        fn clock_nanosleep(clock_id: i32,
                           flags: i32,
                           req: *const timespec,
                           rem: *mut timespec)
                           -> i32;
    }
    match remain {
        Some(p) => unsafe { clock_nanosleep(id, flags, req as *const _, p as *mut _) },
        _ => unsafe { clock_nanosleep(id, flags, req as *const _, 0 as *mut _) },
    }
}

#[cfg(all(target_os = "macos"))]
pub fn clock_nanosleep(_: i32, _: i32, req: &timespec, remain: Option<&mut timespec>) -> i32 {
    extern {
        fn nanosleep(req: *const timespec, rem: *mut timespec) -> i32;
    }
    match remain {
        Some(p) => unsafe { nanosleep(req as *const _, p as *mut _) },
        _ => unsafe { nanosleep(req as *const _, 0 as *mut _) },
    }
}