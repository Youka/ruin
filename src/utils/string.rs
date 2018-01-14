// Windows implementation
#[cfg(windows)]
pub fn str_to_wide(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use std::iter::once;
    OsStr::new(s)   // Convert Rust string into OS native string
        .encode_wide()  // Convert native string into wide char iterator
        .chain(once(0u16)) // Push termination character to the end of iterator
        .collect() // Convert iterator into vector (by return)
}

#[cfg(windows)]
pub fn wide_to_str(w: &[u16]) -> String {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    OsString::from_wide( // Convert unicode codepoints to OS native string
        w.split(|&c| c==0u16).next().unwrap()   // Limit string contents until termination character (excluded)
    ).to_string_lossy().to_string() // Convert native string to Rust string with invalid characters replaced by hints
}

// General implementation
pub fn str_to_cstr(s: &str) -> Vec<u8> {
    use std::ffi::CString;
    CString::new(   // Convert unicode codepoints to OS native string
        s.split('\0').next().unwrap()   // Limit string contents until termination character (excluded)
    ).unwrap().into_bytes_with_nul()    // Convert native string into vector
}

pub fn cstr_to_str(bytes: &[u8]) -> String {
    use std::ffi::CString;
    CString::new(   // Convert unicode codepoints to OS native string
        bytes.split(|&c| c==0u8).next().unwrap()   // Limit string contents until termination character (excluded)
    ).unwrap().to_string_lossy().to_string()    // Convert native string to Rust string with invalid characters replaced by hints
}