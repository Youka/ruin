// Types
pub enum _HWND{}
pub type HWND = *const _HWND;

pub const MB_ICONNONE: u32 = 0x00;
pub const MB_ICONERROR: u32 = 0x10;
pub const MB_ICONQUESTION: u32 = 0x20;
pub const MB_ICONWARNING: u32 = 0x30;
pub const MB_ICONINFORMATION: u32 = 0x40;

pub const MB_OK: u32 = 0x0;
pub const MB_OKCANCEL: u32 = 0x1;
pub const MB_ABORTRETRYIGNORE: u32 = 0x2;
pub const MB_YESNOCANCEL: u32 = 0x3;
pub const MB_YESNO: u32 = 0x4;
pub const MB_RETRYCANCEL: u32 = 0x5;
pub const MB_CANCELTRYCONTINUE: u32 = 0x6;

pub const IDOK: i32 = 1;
pub const IDCANCEL: i32 = 2;
pub const IDABORT: i32 = 3;
pub const IDRETRY: i32 = 4;
pub const IDIGNORE: i32 = 5;
pub const IDYES: i32 = 6;
pub const IDNO: i32 = 7;

// Functions
#[link(name="User32")]
extern {
    pub fn MessageBoxW(hWnd: HWND, lpText: *const u16, lpCaption: *const u16, uType: u32) -> i32;
}