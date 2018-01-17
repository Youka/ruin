#[cfg(windows)]
#[allow(dead_code)]
pub mod winapi;

#[cfg(target_os = "linux")]
#[allow(dead_code)]
pub mod gtk;

#[cfg(target_os = "macos")]
#[allow(dead_code)]
pub mod cocoa;