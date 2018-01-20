// Types
use std::collections::BTreeMap;
pub type Catalog = BTreeMap<String,String>;
pub type Register = BTreeMap<String,Catalog>;

pub enum Error{
    NoRegister,
    NoCatalog
}
use std::fmt::{Display, Formatter, Result as FmtResult};
impl Display for Error{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Error::NoRegister => write!(f, "No register available!"),
            Error::NoCatalog => write!(f, "No catalog available!")
        }
    }
}

// Catalog I/O
use std::io::Read;
pub fn load_catalog_from_reader(reader: &mut Read) -> Catalog {
    let mut catalog = Catalog::new();
    use std::io::{BufReader,BufRead};
    for line_result in BufReader::new(reader).lines() {
        if let Ok(mut line) = line_result {
            if let Some(sep) = line.find('#') {
                line.truncate(sep);
            }
            if let Some(sep) = line.find('=') {
                let (id, message) = line.split_at(sep);
                catalog.insert(String::from(id), String::from(message));
            }
        }
    }
    catalog
}
use std::path::Path;
use std::io::Result as IoResult;
pub fn load_catalog_from_file<P: AsRef<Path>>(path: P) -> IoResult<Catalog> {
    use std::fs::File;
    Ok(load_catalog_from_reader(&mut File::open(path)?))
}

use std::io::Write;
pub fn save_catalog_to_writer<'a>(catalog: &'a Catalog, writer: &mut Write) -> IoResult<&'a Catalog> {
    use std::io::BufWriter;
    let mut writer = BufWriter::new(writer);
    for (id, message) in catalog {
        let mut entry = id.clone();
        entry.push('=');
        entry.push_str(message);
        entry.push('\n');
        writer.write(entry.as_bytes())?;
    }
    Ok(catalog)
}
pub fn save_catalog_to_file<P: AsRef<Path>>(catalog: &Catalog, path: P) -> IoResult<&Catalog> {
    use std::fs::OpenOptions;
    save_catalog_to_writer(catalog, &mut OpenOptions::new().write(true).truncate(true).create(true).open(path)?)
}

// Register I/O
pub fn load_register_from_directory<P: AsRef<Path>>(path: P) -> IoResult<Register> {
    let mut register = Register::new();
    use std::fs::read_dir;
    for dir_entry_result in read_dir(path)? {
        if let Ok(dir_entry) = dir_entry_result {
            register.insert(
                dir_entry.file_name().to_string_lossy().to_string(),
                load_catalog_from_file(dir_entry.path().to_string_lossy().to_string())?
            );
        }
    }
    Ok(register)
}
pub fn save_register_to_directory<P: AsRef<Path>>(register: &Register, path: P) -> IoResult<&Register> {
    use std::fs::create_dir;
    create_dir(&path).is_ok();
    for (catalog_name, catalog) in register {
        save_catalog_to_file(catalog, path.as_ref().clone().join(catalog_name))?;
    }
    Ok(register)
}

// Thread register & catalog
use std::cell::RefCell;
struct RegisterState {
    register: Register,
    catalog: Option<Catalog>
}
thread_local!(
    static ACTIVE_REGISTER_STATE: RefCell<Option<RegisterState>> = RefCell::new(Option::None);
);
pub fn set_active_register(register: Register) {
    ACTIVE_REGISTER_STATE.with(|state|{
        *state.borrow_mut() = Some(RegisterState{register, catalog: None});
    });
}
pub fn set_active_catalog(catalog_name: &str) -> Result<&str,Error> {
    ACTIVE_REGISTER_STATE.with(|state|{
        if let Some(ref mut state) = *state.borrow_mut() {
            if let Some(catalog) = state.register.get_mut(&String::from(catalog_name)) {
                state.catalog = Some(catalog.clone());
                return Ok(catalog_name);
            }
            return Err(Error::NoCatalog);
        }
        Err(Error::NoRegister)
    })
}
pub fn available_catalogs() -> Result<Vec<String>,Error> {
    ACTIVE_REGISTER_STATE.with(|state|{
        if let Some(ref state) = *state.borrow() {
            return Ok(state.register.keys().cloned().collect());
        }
        Err(Error::NoCatalog)
    })
}

// Thread-local localization
#[allow(dead_code)]
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
    #[test]
    fn write_catalog() {
        use super::{Catalog,save_catalog_to_writer};
        let mut catalog = Catalog::new();
        catalog.insert(String::from("test"), String::from("Foo"));
        catalog.insert(String::from("test2"), String::from("Bar"));
        let mut writer_target: Vec<u8> = Vec::new();
        {
            use std::io::BufWriter;
            let mut writer = BufWriter::new(&mut writer_target);
            save_catalog_to_writer(&catalog, &mut writer).expect("Saving catalog to writer failed!");
        }
        assert_eq!(
            String::from_utf8_lossy(&writer_target).to_string(),
            String::from("test=Foo\ntest2=Bar\n")
        );
    }
}