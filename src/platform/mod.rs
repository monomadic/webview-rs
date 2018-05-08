pub use self::platform::*;

#[cfg(target_os = "macos")]
#[path="mac/mod.rs"]
pub mod platform;

#[cfg(target_os = "windows")]
#[path="win/mod.rs"]
pub mod platform;
