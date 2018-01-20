// Check platform
#[cfg(any(target_os = "macos", target_os = "ios"))]
#[test]
fn check_platform() {
    assert!(false, "What the heck?! Don't support Apple -.-");
}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
#[test]
fn check_platform() {
}