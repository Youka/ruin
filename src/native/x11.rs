// Types
pub enum _Display{}
pub type Display = *const _Display;
pub enum _Window{}
pub type Window = *const _Window;
#[repr(C)]
pub union XEvent {
    pub typ: i32,
    reserved: [u8;256], // Placeholder for event kinds, TBC
    pad: [u32;24]
}
pub type Atom = u32;
pub type Status = i32;

pub const EXPOSURE_MASK: i32 = (1u32<<15) as i32;
pub const KEY_PRESS_MASK: i32 = (1u32<<0) as i32;
pub const KEY_PRESS: i32 = 2;
pub const CLIENT_MESSAGE: i32 = 33;

// Functions
#[link(name="X11")]
extern {
    pub fn XOpenDisplay(display_name: *const u8) -> Display;
    pub fn XCloseDisplay(display: Display);
    pub fn XDefaultScreen(display: Display) -> i32;
    pub fn XCreateSimpleWindow(
        display: Display, parent: Window,
        x: i32, y: i32,
        width: u32, height: u32,
        border_width: u32,
        border: u32, background: u32
    ) -> Window;
    pub fn XStoreName(display: Display, w: Window, window_name: *const u8);
    pub fn XRootWindow(display: Display, screen: i32) -> Window;
    pub fn XBlackPixel(display: Display, screen: i32) -> u32;
    pub fn XWhitePixel(display: Display, screen: i32) -> u32;
    pub fn XSelectInput(display: Display, w: Window, event_mask: i32);
    pub fn XMapWindow(display: Display, w: Window);
    pub fn XInternAtom(display: Display, name: *const u8, only_if_exists: bool) -> Atom;
    pub fn XSetWMProtocols(display: Display, w: Window, atom: *const Atom, count: i32) -> Status;
    pub fn XNextEvent(display: Display, e: *mut XEvent) -> i32;
}