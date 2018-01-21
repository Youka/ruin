extern crate ruin;

fn main() {
    use ruin::gui::dialog::{messagebox,Icon,Buttons};
    println!("{:?}", messagebox("Hello wörld!", "ruin", Icon::Question, Buttons::OkCancel));
}