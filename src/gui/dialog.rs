// Windows implementation
#[cfg(windows)]
mod native{
    // C bindings
    #[link(name="User32")]
    extern {
        fn MessageBoxW(hWnd: usize, lpText: *const u16, lpCaption: *const u16, uType: u32) -> i32;
    }

    // C wrappers
    pub fn messagebox(text: &str, caption: &str) {
        use ::utils::string::str_to_wide;
        let text_wide = str_to_wide(text);
        let caption_wide = str_to_wide(caption);
        unsafe {
            MessageBoxW(0, text_wide.as_ptr(), caption_wide.as_ptr(), 0);
        }
    }
}

// Unix implementation
#[cfg(unix)]
mod native{
    // C bindings
    enum _Display{}
    type Display = *const _Display;
    enum _Window{}
    type Window = *const _Window;
    #[repr(C)]
    union XEvent {
        typ: i32,
        reserved: [u8;256], // Placeholder for event kinds, TBC
        pad: [u32;24]
    }
    type Atom = u32;
    type Status = i32;

    const EXPOSURE_MASK: i32 = (1u32<<15) as i32;
    const KEY_PRESS_MASK: i32 = (1u32<<0) as i32;
    const KEY_PRESS: i32 = 2;
    const CLIENT_MESSAGE: i32 = 33;

    #[link(name="X11")]
    extern {
        fn XOpenDisplay(display_name: *const u8) -> Display;
        fn XCloseDisplay(display: Display);
        fn XDefaultScreen(display: Display) -> i32;
        fn XCreateSimpleWindow(
            display: Display, parent: Window,
            x: i32, y: i32,
            width: u32, height: u32,
            border_width: u32,
            border: u32, background: u32
        ) -> Window;
        fn XStoreName(display: Display, w: Window, window_name: *const u8);
        fn XRootWindow(display: Display, screen: i32) -> Window;
        fn XBlackPixel(display: Display, screen: i32) -> u32;
        fn XWhitePixel(display: Display, screen: i32) -> u32;
        fn XSelectInput(display: Display, w: Window, event_mask: i32);
        fn XMapWindow(display: Display, w: Window);
        fn XInternAtom(display: Display, name: *const u8, only_if_exists: bool) -> Atom;
        fn XSetWMProtocols(display: Display, w: Window, atom: *const Atom, count: i32) -> Status;
        fn XNextEvent(display: Display, e: *mut XEvent) -> i32;
    }

    // C wrappers
    pub fn messagebox(text: &str, caption: &str) {
        use std::ptr::null;
        use ::utils::string::str_to_cstr;
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
}

// Native wrappers
pub fn messagebox(text: &str, caption: &str) {
    native::messagebox(text, caption);
}