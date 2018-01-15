// messagebox
#[cfg(windows)]
pub fn messagebox(text: &str, caption: &str) {
    use ::utils::string::str_to_wide;
    use ::native::winapi::MessageBoxW;
    let text_wide = str_to_wide(text);
    let caption_wide = str_to_wide(caption);
    unsafe {
        MessageBoxW(0, text_wide.as_ptr(), caption_wide.as_ptr(), 0);
    }
}

#[cfg(unix)]
pub fn messagebox(text: &str, caption: &str) {
    use std::ptr::null;
    use ::utils::string::str_to_cstr;
    use ::native::x11::*;
    unsafe{
        // Load display & default screen
        let display = XOpenDisplay(null());
        if display == null() {
            panic!("Can't open a display!");
        }
        let screen = XDefaultScreen(display);
        // Create window on screen
        let window = XCreateSimpleWindow(display, XRootWindow(display, screen), 0, 0, 400, 400, 1, XBlackPixel(display, screen), XWhitePixel(display, screen));
        if window == null() {
            panic!("Can't create a window!");
        }
        let window_name = str_to_cstr(&[caption, text].join(" - "));
        XStoreName(display, window, window_name.as_ptr());
        // Select events to handle by window
        XSelectInput(display, window, EXPOSURE_MASK | KEY_PRESS_MASK);
        XMapWindow(display, window);
        let wm_delete_window = str_to_cstr("WM_DELETE_WINDOW");
        let atom = XInternAtom(display, wm_delete_window.as_ptr(), false);
        XSetWMProtocols(display, window, &atom, 1);
        // Process window events
        let mut event = XEvent{typ: 0};
        loop {
            XNextEvent(display, &mut event);
            if event.typ == KEY_PRESS || event.typ == CLIENT_MESSAGE {
                break;
            }
        }
        // Unload display and free child resources
        XCloseDisplay(display);
    }
}