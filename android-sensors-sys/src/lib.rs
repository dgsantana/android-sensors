mod logging;
mod sensors;

#[cfg(target_os = "android")]
pub mod ffi {
    pub mod sensors {
        pub use crate::sensors::*;
    }

    pub mod logging {
        pub use crate::logging::*;
    }
}
