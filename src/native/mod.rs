#[cfg(windows)]
pub mod winapi;

#[cfg(unix)]
pub mod x11;