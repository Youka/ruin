// Types
pub enum _GtkWidget{}
pub type GtkWidget = *const _GtkWidget;

pub const GTK_DIALOG_MODAL: u32 = 1 << 0;
pub const GTK_DIALOG_DESTROY_WITH_PARENT: u32 = 1 << 1;
pub const GTK_DIALOG_USE_HEADER_BAR: u32 = 1 << 2;

pub const GTK_MESSAGE_INFO: u32 = 0;
pub const GTK_MESSAGE_WARNING: u32 = 1;
pub const GTK_MESSAGE_QUESTION: u32 = 2;
pub const GTK_MESSAGE_ERROR: u32 = 3;
pub const GTK_MESSAGE_OTHER: u32 = 4;

pub const GTK_BUTTONS_NONE: u32 = 0;
pub const GTK_BUTTONS_OK: u32 = 1;
pub const GTK_BUTTONS_CLOSE: u32 = 2;
pub const GTK_BUTTONS_CANCEL: u32 = 3;
pub const GTK_BUTTONS_YES_NO: u32 = 4;
pub const GTK_BUTTONS_OK_CANCEL: u32 = 5;

pub const GTK_RESPONSE_NONE: i32 = -1;
pub const GTK_RESPONSE_REJECT: i32 = -2;
pub const GTK_RESPONSE_ACCEPT: i32 = -3;
pub const GTK_RESPONSE_DELETE_EVENT: i32 = -4;
pub const GTK_RESPONSE_OK: i32 = -5;
pub const GTK_RESPONSE_CANCEL: i32 = -6;
pub const GTK_RESPONSE_CLOSE: i32 = -7;
pub const GTK_RESPONSE_YES: i32 = -8;
pub const GTK_RESPONSE_NO: i32 = -9;
pub const GTK_RESPONSE_APPLY: i32 = -10;
pub const GTK_RESPONSE_HELP: i32 = -11;

// Functions
#[link(name="gtk-3")]
extern {
    pub fn gtk_init_check(argc: i32, argv: *const u8) -> bool;
    pub fn gtk_message_dialog_new(parent: GtkWidget, flags: u32, typ: u32, buttons: u32, message: *const u8) -> GtkWidget;
    pub fn gtk_widget_destroy(widget: GtkWidget);
    pub fn gtk_dialog_add_button (dialog: GtkWidget, button_text: *const u8, response_id: i32) -> GtkWidget;
    pub fn gtk_window_set_title(window: GtkWidget, title: *const u8);
    pub fn gtk_window_set_keep_above(window: GtkWidget, setting: bool);
    pub fn gtk_dialog_run(dialog: GtkWidget) -> i32;
}