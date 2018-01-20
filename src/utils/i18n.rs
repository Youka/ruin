// Types
use std::collections::BTreeMap;
pub type Catalog = BTreeMap<String,String>;
pub type Register = BTreeMap<String,Catalog>;

// Catalog I/O
pub fn load_catalog_from_reader() {

    // TODO
    unimplemented!();

}
pub fn load_catalog_from_file() {

    // TODO
    unimplemented!();

}

pub fn save_catalog_to_writer() {

    // TODO
    unimplemented!();

}
pub fn save_catalog_to_file() {

    // TODO
    unimplemented!();

}

// Register I/O
pub fn load_register_from_directory() {

    // TODO
    unimplemented!();

}
pub fn save_register_to_directory() {

    // TODO
    unimplemented!();

}

// Thread register & catalog
use std::cell::RefCell;
thread_local!(
    static ACTIVE_REGISTER: RefCell<Option<Register>> = RefCell::new(Option::None);
    static ACTIVE_CATALOG: RefCell<Option<Catalog>> = RefCell::new(Option::None);
);
pub fn set_active_register(register: Register) {
    ACTIVE_REGISTER.with(|reg|{
        *reg.borrow_mut() = Some(register);
    });
    ACTIVE_CATALOG.with(|cat|{
        *cat.borrow_mut() = None;
    });
}
pub fn set_active_catalog(catalog: &str) -> bool {
    ACTIVE_REGISTER.with(|reg|{
        if let Some(ref reg) = *reg.borrow() {
            if let Some(catalog) = reg.get(&String::from(catalog)) {
                ACTIVE_CATALOG.with(|cat|{
                    *cat.borrow_mut() = Some(catalog.clone());
                });
                return true;
            }
        }
        false
    })
}
pub fn available_catalogs() -> Result<Vec<String>,&'static str> {
    ACTIVE_REGISTER.with(|reg|{
        if let Some(ref reg) = *reg.borrow() {
            return Ok(reg.keys().cloned().collect());
        }
        Err("No register available!")
    })
}

// Thread-local localization
fn format_message() {

    // TODO
    unimplemented!();

}

#[macro_export]
macro_rules! tl {
    ($origin:expr) => (

        // TODO

    );
    ($origin:expr, $inserts:expr) => (

        // TODO

    );
}


// Tests
#[cfg(test)]
mod tests {

    // TODO

}