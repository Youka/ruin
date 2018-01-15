// Functions
#[link(name="User32")]
extern {
    pub fn MessageBoxW(hWnd: usize, lpText: *const u16, lpCaption: *const u16, uType: u32) -> i32;
}