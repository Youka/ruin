// Nothing to do on windows
#[cfg(windows)]
fn main() {
}

// Detect X11 (dev) system library on unix
#[cfg(unix)]
fn main() {
    use std::process::Command;
    if String::from_utf8_lossy(
        &Command::new("gcc")
            .arg("-lX11")
            .output().expect("Couldn't run GCC to check X11 library existence!")
            .stderr
    ).to_string().contains("-lX11") {
        panic!("X11 development library not found! Install libx11-dev to continue.");
    }
}