// Shortcut library namespace
use super::*;


// Test utils
#[cfg(windows)]
#[test]
fn wide_convert() {
    use utils::string::{str_to_wide,wide_to_str};
    const TEXT: &str = "This is an unicode string test!!!";
    assert_eq!(
        wide_to_str(&str_to_wide(TEXT)[..]),
        String::from(TEXT)
    );
}

#[test]
fn cstr_convert() {
    use utils::string::{str_to_cstr,cstr_to_str};
    assert_eq!(
        cstr_to_str(&str_to_cstr("This is a cstring\0test!!!")[..]),
        String::from("This is a cstring")
    );
}

// Test gui
#[test]
fn messagebox() {
    use gui::dialog::messagebox;
    messagebox("Hello world!", "Test");
}


// Personal urge
#[cfg(any(target_os = "macos", target_os = "ios"))]
#[test]
fn check_sanity() {
    assert!(false, "What the heck?! Don't support Apple -.-");
}