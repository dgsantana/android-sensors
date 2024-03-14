#[cfg(target_os = "android")]
pub mod looper;
#[cfg(target_os = "android")]
pub mod manager;
#[cfg(target_os = "android")]
pub mod sensor;
#[cfg(target_os = "android")]
pub mod sensor_queue;
#[cfg(target_os = "android")]
mod utils;