// Nothing to do on windows & mac
#[cfg(not(target_os = "linux"))]
fn main() {
}

// Detect GTK+ 3 (dev) library on linux
#[cfg(target_os = "linux")]
fn main() {
    check_library("GTK+ 3", "gtk-3");
}


// Detect system library with GCC
#[cfg(target_os = "linux")]
fn check_library(lib_name: &str, lib_cmd: &str) {
    use std::process::Command;
    let link_arg = &(String::from("-l") + lib_cmd);
    if String::from_utf8_lossy(
        &Command::new("gcc")
            .arg(link_arg)
            .output().expect(&format!("Couldn't run GCC to check {} library existence!", lib_name))
            .stderr
    ).to_string().contains(link_arg) {
        panic!(format!("{} development library not found! Install lib{}-dev and retry.", lib_name, lib_cmd));
    }
}