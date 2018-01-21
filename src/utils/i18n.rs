// Types
use std::collections::BTreeMap;
pub type Catalog = BTreeMap<String,String>;
pub type Register = BTreeMap<String,Catalog>;

#[derive(Debug)]
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
use std::io::Result as IoResult;
pub fn load_catalog(reader: &mut Read) -> IoResult<Catalog> {
    let mut catalog = Catalog::new();
    use std::io::{BufReader,BufRead};
    for line_result in BufReader::new(reader).lines() {
        let mut line = try!(line_result);
        if let Some(sep) = line.find('#') {
            line.truncate(sep);
        }
        if let Some(sep) = line.find('=') {
            catalog.insert(String::from(&line[..sep]), String::from(&line[sep+1..]));
        }
    }
    Ok(catalog)
}
pub fn load_catalog_data(data: &[u8]) -> IoResult<Catalog> {
    use std::io::Cursor;
    load_catalog(&mut Cursor::new(data))
}
use std::path::Path;
pub fn load_catalog_file<P: AsRef<Path>>(path: P) -> IoResult<Catalog> {
    use std::fs::File;
    load_catalog(&mut File::open(path)?)
}

use std::io::Write;
pub fn save_catalog<'a>(catalog: &'a Catalog, writer: &mut Write) -> IoResult<&'a Catalog> {
    use std::io::BufWriter;
    let mut writer = BufWriter::new(writer);
    for (id, message) in catalog {
        let mut entry = String::with_capacity(id.len() + message.len() + 2);
        entry.push_str(id);
        entry.push('=');
        entry.push_str(message);
        entry.push('\n');
        writer.write(entry.as_bytes())?;
    }
    Ok(catalog)
}
pub fn save_catalog_data<'a>(catalog: &'a Catalog, data: &mut Vec<u8>) -> IoResult<&'a Catalog> {
    use std::io::BufWriter;
    save_catalog(&catalog, &mut BufWriter::new(data))
}
pub fn save_catalog_file<P: AsRef<Path>>(catalog: &Catalog, path: P) -> IoResult<&Catalog> {
    use std::fs::OpenOptions;
    save_catalog(catalog, &mut OpenOptions::new().write(true).truncate(true).create(true).open(path)?)
}

// Register I/O
pub fn load_register<P: AsRef<Path>>(path: P) -> IoResult<Register> {
    let mut register = Register::new();
    use std::fs::read_dir;
    for dir_entry_result in read_dir(path)? {
        let dir_entry = try!(dir_entry_result);
        register.insert(
            dir_entry.file_name().to_string_lossy().to_string(),
            load_catalog_file(dir_entry.path().to_string_lossy().to_string())?
        );
    }
    Ok(register)
}
pub fn save_register<P: AsRef<Path>>(register: &Register, path: P) -> IoResult<&Register> {
    use std::fs::create_dir;
    create_dir(&path).is_ok();
    for (catalog_name, catalog) in register {
        save_catalog_file(catalog, path.as_ref().clone().join(catalog_name))?;
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

// Thread localization
pub fn available_catalogs() -> Result<Vec<String>,Error> {
    ACTIVE_REGISTER_STATE.with(|state|{
        if let Some(ref state) = *state.borrow() {
            return Ok(state.register.keys().cloned().collect());
        }
        Err(Error::NoRegister)
    })
}

pub fn format_message(message: &str, tokens: &[&str]) -> String {
    let mut message = message.replace("\\n", "\n").replace("\\{}", "\0");
    for token in tokens {
        if let Some(placeholder) = message.find("{}") {
            message = String::from(&message[..placeholder]) + token + &message[placeholder+2..];
        }else{
            break;
        }
    }
    message.replace("\0", "{}")
}
pub fn translate(id: &str, tokens: &[&str]) -> Result<Option<String>,Error> {
    ACTIVE_REGISTER_STATE.with(|state|{
        if let Some(ref state) = *state.borrow() {
            if let Some(ref catalog) = state.catalog {
                if let Some(message) = catalog.get(id) {
                    return Ok(Some(format_message(message, tokens)));
                }
                return Ok(None);
            }
            return Err(Error::NoCatalog);
        }
        Err(Error::NoRegister)
    })
}
#[macro_export]
macro_rules! tl {
    ($origin:expr) => (
        tl!($origin,)
    );
    ($origin:expr,$( $token:expr ),*) => (
        {
            #[allow(unused_mut)]
            let mut tokens = Vec::new();
            $(
                tokens.push($token);
            )*
            if let Ok(Some(msg)) = $crate::utils::i18n::translate($origin , &tokens) {
                msg
            }else{
                String::from($origin)
            }
        }
    );
}


// Tests
#[cfg(test)]
mod tests {
    #[test]
    fn write_read_catalog() {
        use super::{Catalog,save_catalog_data,load_catalog_data};
        // Template
        let mut catalog = Catalog::new();
        catalog.insert(String::from("test"), String::from("Foo"));
        catalog.insert(String::from("test2"), String::from("Bar"));
        // Write
        let mut buffer: Vec<u8> = Vec::new();
        save_catalog_data(&catalog, &mut buffer).expect("Saving catalog to writer failed!");
        assert_eq!(
            String::from_utf8_lossy(&buffer).to_string(),
            String::from("test=Foo\ntest2=Bar\n")
        );
        // Read
        assert_eq!(
            load_catalog_data(&buffer).expect("Loading catalog from reader failed!"),
            catalog
        );
    }

    #[test]
    fn check_translate() {
        use super::{Register,Catalog,set_active_register,available_catalogs,set_active_catalog};
        use std::iter::{once,FromIterator};
        set_active_register(
            Register::from_iter(once(
                (
                    String::from("de-de"),
                    Catalog::from_iter(once(
                        (
                            String::from("id"),
                            String::from("{}\\n\\{}")
                        )
                    ))
                )
            ))
        );
        assert!(available_catalogs().expect("Register wasn't set previously?!").contains(&String::from("de-de")), "But... but the catalog must be there!");
        set_active_catalog("de-de").is_ok();
        assert_eq!(tl!("id"), String::from("{}\n{}"));
        set_active_catalog("en-en").is_ok();
        assert_eq!(tl!("Blub"), String::from("Blub"));
    }
}