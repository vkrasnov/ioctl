use std::os::raw::{c_int, c_ulong};

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[macro_use]
mod platform;

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub use platform::*;

extern "C" {
    #[doc(hidden)]
    pub fn ioctl(fd: c_int, req: c_ulong, ...) -> c_int;
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
use platform_not_supported;
