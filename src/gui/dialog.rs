/// Icons to display on message dialog.
pub enum Icon{
    Info,
    Question,
    Warning,
    Error
}

/// Buttons on message dialog.
pub enum Buttons{
    Ok,
    OkCancel,
    YesNo,
    AbortRetryIgnore
}

/// Triggered button on message dialog.
#[derive(Debug)]
pub enum Button{
    Ok,
    Cancel,
    Yes,
    No,
    Abort,
    Retry,
    Ignore
}

/// Shows modal message dialog with custom window caption and message text.
#[cfg(windows)]
pub fn messagebox(text: &str, caption: &str, icon: Icon, buttons: Buttons) -> Option<Button> {
    use utils::string::str_to_wide;
    use native::winapi::*;
    use std::ptr::null;
    let text_wide = str_to_wide(text);
    let caption_wide = str_to_wide(caption);
    unsafe {
        match MessageBoxW(
            null(), text_wide.as_ptr(), caption_wide.as_ptr(),
            match icon {
                Icon::Info => MB_ICONINFORMATION,
                Icon::Question => MB_ICONQUESTION,
                Icon::Warning => MB_ICONWARNING,
                Icon::Error => MB_ICONERROR
            } + match buttons {
                Buttons::Ok => MB_OK,
                Buttons::OkCancel => MB_OKCANCEL,
                Buttons::YesNo => MB_YESNO,
                Buttons::AbortRetryIgnore => MB_ABORTRETRYIGNORE
            }
        ){
            IDOK => Some(Button::Ok),
            IDCANCEL => Some(Button::Cancel),
            IDYES => Some(Button::Yes),
            IDNO => Some(Button::No),
            IDABORT => Some(Button::Abort),
            IDRETRY => Some(Button::Retry),
            IDIGNORE => Some(Button::Ignore),
            _ => None
        }
    }
}
/// Shows modal message dialog with custom window caption and message text.
#[cfg(target_os = "linux")]
pub fn messagebox(text: &str, caption: &str, icon: Icon, buttons: Buttons) {
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
/// Shows modal message dialog with custom window caption and message text.
#[cfg(target_os = "macos")]
#[allow(unused)]
pub fn messagebox(text: &str, caption: &str, icon: Icon, buttons: Buttons) {
    unimplemented!();
}