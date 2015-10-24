//! nanosleep for rust

#![crate_type = "lib"]

#![crate_name = "shuteye"]

use std::fmt;

#[repr(C)]
#[derive(Copy, Clone, PartialEq)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

impl fmt::Debug for timespec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "timespec(sec: {}, nsec: {})", self.tv_sec, self.tv_nsec)
    }
}

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
