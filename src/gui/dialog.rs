/// Icons to display on message dialog.
#[derive(PartialEq, Eq, Debug)]
pub enum Icon{
    None,
    Info,
    Question,
    Warning,
    Error
}
/// Buttons on message dialog.
#[derive(PartialEq, Eq, Debug)]
pub enum Buttons{
    Ok,
    OkCancel,
    YesNo,
    AbortRetryIgnore
}
/// Triggered button on message dialog.
#[derive(PartialEq, Eq, Debug)]
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
                Icon::None => MB_ICONNONE,
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
pub fn messagebox(text: &str, caption: &str, icon: Icon, buttons: Buttons) -> Option<Button> {
    use native::gtk::*;
    use std::ptr::null;
    use utils::string::str_to_cstr;
    let text_c = str_to_cstr(text);
    let caption_c = str_to_cstr(caption);
    let abort_button_label = str_to_cstr(&tl!("Abort"));
    let retry_button_label = str_to_cstr(&tl!("Retry"));
    let ignore_button_label = str_to_cstr(&tl!("Ignore"));
    unsafe{
        if !gtk_init_check(0, null()) {
            panic!("Couldn't initialize GTK!");
        }
        let dialog = gtk_message_dialog_new(
            null(), GTK_DIALOG_MODAL,
            match icon {
                Icon::None => GTK_MESSAGE_OTHER,
                Icon::Info => GTK_MESSAGE_INFO,
                Icon::Question => GTK_MESSAGE_QUESTION,
                Icon::Warning => GTK_MESSAGE_WARNING,
                Icon::Error => GTK_MESSAGE_ERROR
            },
            match buttons {
                Buttons::Ok => GTK_BUTTONS_OK,
                Buttons::OkCancel => GTK_BUTTONS_OK_CANCEL,
                Buttons::YesNo => GTK_BUTTONS_YES_NO,
                Buttons::AbortRetryIgnore => GTK_BUTTONS_NONE
            },
            text_c.as_ptr()
        );
        const RESPONSE_ABORT: i32 = -100;
        const RESPONSE_RETRY: i32 = -101;
        const RESPONSE_IGNORE: i32 = -102;
        if buttons == Buttons::AbortRetryIgnore {
            gtk_dialog_add_button(dialog, abort_button_label.as_ptr(), RESPONSE_ABORT);
            gtk_dialog_add_button(dialog, retry_button_label.as_ptr(), RESPONSE_RETRY);
            gtk_dialog_add_button(dialog, ignore_button_label.as_ptr(), RESPONSE_IGNORE);
        }
        gtk_window_set_title(dialog, caption_c.as_ptr());
        gtk_window_set_keep_above(dialog, true);
        let response = gtk_dialog_run(dialog);
        gtk_widget_destroy(dialog);
        match response {
            GTK_RESPONSE_OK => Some(Button::Ok),
            GTK_RESPONSE_CANCEL => Some(Button::Cancel),
            GTK_RESPONSE_YES => Some(Button::Yes),
            GTK_RESPONSE_NO => Some(Button::No),
            RESPONSE_ABORT => Some(Button::Abort),
            RESPONSE_RETRY => Some(Button::Retry),
            RESPONSE_IGNORE => Some(Button::Ignore),
            _ => None
        }
    }
}
/// Shows modal message dialog with custom window caption and message text.
#[cfg(target_os = "macos")]
#[allow(unused)]
pub fn messagebox(text: &str, caption: &str, icon: Icon, buttons: Buttons) -> Option<Button> {

    // TODO
    unimplemented!();
    
}