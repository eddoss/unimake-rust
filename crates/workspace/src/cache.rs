use serde::Serialize;
use serde::de::DeserializeOwned;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::path::PathBuf;
use std::rc::Rc;
use std::str::FromStr;

//////////////////////////////////////////////////////////////////
// Cache holder
//////////////////////////////////////////////////////////////////

pub struct Cache {
    pub cli: Entry<umk::Cli>,
}

impl Cache {
    pub fn new(keys: Keys, driver: Rc<RefCell<dyn Driver>>) -> Self {
        Self {
            cli: Entry::<umk::Cli> {
                key: keys.cli,
                driver: driver.clone(),
                _phantom: PhantomData,
            },
        }
    }
}

//////////////////////////////////////////////////////////////////
// Cache keys
//////////////////////////////////////////////////////////////////

pub struct Keys {
    pub cli: String,
}

impl Default for Keys {
    fn default() -> Self {
        Self {
            cli: String::from("cli"),
        }
    }
}
//////////////////////////////////////////////////////////////////
// Json IO driver trait
//////////////////////////////////////////////////////////////////

type JsonValue = serde_json::Value;

pub trait Driver {
    fn read(&self, key: &str) -> umk::Result<Option<JsonValue>>;
    fn write(&mut self, key: &str, value: JsonValue) -> umk::Result;
    fn remove(&mut self, key: &str) -> umk::Result;
}

//////////////////////////////////////////////////////////////////
// Cache entry
//////////////////////////////////////////////////////////////////

pub struct Entry<T>
where
    T: DeserializeOwned,
{
    key: String,
    driver: Rc<RefCell<dyn Driver>>,
    _phantom: PhantomData<T>,
}

impl<T: DeserializeOwned> Entry<T> {
    pub fn get(&self) -> umk::Result<Option<T>> {
        let driver = self.driver.borrow();
        let value = driver.read(self.key.as_str())?;
        match value {
            None => Ok(None),
            Some(value) => {
                let result: Result<T, serde_json::Error> = serde_json::from_value(value);
                match result {
                    Err(e) => Err(umk::Error::new(e.to_string().as_str())),
                    Ok(v) => Ok(Some(v)),
                }
            }
        }
    }

    pub fn set<V: Serialize>(&self, value: V) -> umk::Result {
        let json_value = serde_json::to_value(value);
        match json_value {
            Err(e) => Err(umk::Error::new(e.to_string().as_str())),
            Ok(v) => self.driver.borrow_mut().write(self.key.as_str(), v),
        }
    }

    pub fn del(&self, key: &str) -> umk::Result {
        self.driver.borrow_mut().remove(key)
    }
}

//////////////////////////////////////////////////////////////////
// Filesystem driver
//////////////////////////////////////////////////////////////////

pub struct FilesystemDriver {
    root: PathBuf,
}

impl FilesystemDriver {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
}

impl Driver for FilesystemDriver {
    fn read(&self, key: &str) -> umk::Result<Option<JsonValue>> {
        let file = self.root.join(key);
        if !file.exists() {
            return Ok(None);
        }
        match std::fs::read_to_string(file) {
            Err(e) => Err(umk::Error::new(e.to_string().as_str())),
            Ok(v) => match JsonValue::from_str(&v) {
                Err(e) => Err(umk::Error::new(e.to_string().as_str())),
                Ok(v) => Ok(Some(v)),
            },
        }
    }

    fn write(&mut self, key: &str, value: JsonValue) -> umk::Result {
        let file = self.root.join(key);
        if !self.root.exists() {
            if let Err(e) = std::fs::create_dir_all(&self.root) {
                return Err(umk::Error::new(e.to_string().as_str()));
            }
        }
        match serde_json::to_value(value) {
            Err(e) => Err(umk::Error::new(e.to_string().as_str())),
            Ok(v) => match serde_json::to_string_pretty(&v) {
                Err(e) => Err(umk::Error::from_error(e)),
                Ok(v) => match std::fs::write(file, v) {
                    Err(e) => Err(umk::Error::from_error(e)),
                    _ => Ok(()),
                },
            },
        }
    }

    fn remove(&mut self, key: &str) -> umk::Result {
        let result = std::fs::remove_file(self.root.join(key));
        if let Err(e) = result {
            return if e.kind() != std::io::ErrorKind::NotFound {
                Ok(())
            } else {
                Err(umk::Error::from_io_error(&e))
            };
        }
        Ok(())
    }
}
