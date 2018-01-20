// messagebox
#[cfg(windows)]
pub fn messagebox(text: &str, caption: &str) {
    use utils::string::str_to_wide;
    use native::winapi::*;
    use std::ptr::null;
    let text_wide = str_to_wide(text);
    let caption_wide = str_to_wide(caption);
    unsafe {
        MessageBoxW(null(), text_wide.as_ptr(), caption_wide.as_ptr(), MB_ICONINFORMATION + MB_OK);
    }
}

#[cfg(target_os = "linux")]
pub fn messagebox(text: &str, caption: &str) {
    use native::gtk::*;
    use std::ptr::null;
    use utils::string::str_to_cstr;
    let text_c = str_to_cstr(text);
    let caption_c = str_to_cstr(caption);
    unsafe{
        if !gtk_init_check(0, null()) {
            panic!("Couldn't initialize GTK!");
        }
        let dialog = gtk_message_dialog_new(null(), GTK_DIALOG_MODAL, GTK_MESSAGE_INFO, GTK_BUTTONS_OK, text_c.as_ptr());
        gtk_window_set_title(dialog, caption_c.as_ptr());
        gtk_window_set_keep_above(dialog, true);
        gtk_dialog_run(dialog);
        gtk_widget_destroy(dialog);
    }
}

#[cfg(target_os = "macos")]
#[allow(unused)]
pub fn messagebox(text: &str, caption: &str) {
    unimplemented!();
}