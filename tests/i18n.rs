#[macro_use]
extern crate ruin;

#[test]
fn translate_something() {
    use ruin::utils::i18n::{load_register,activate_register,choose_catalog};
    activate_register(load_register("./tests/i18n").expect("I18n directory not found!"));
    choose_catalog("en-us").expect("Catalog 'en-us' not found?!");
    assert_eq!(tl!("test", "Hello", "world"), "Hello, world!\n");
}