use crate::Result;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

//////////////////////////////////////////////////////////////////
// Json IO driver trait
//////////////////////////////////////////////////////////////////

type JsonValue = serde_json::Value;

pub trait Driver: Send + Sync + 'static {
    fn get(&self, key: &str) -> Result<Option<JsonValue>>;
    fn set(&mut self, key: &str, value: JsonValue) -> Result;
    fn del(&mut self, key: &str) -> Result;
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
    pub fn get(&self) -> Result<Option<T>> {
        match self.driver.borrow().get(self.key.as_str())? {
            None => Ok(None),
            Some(value) => Ok(Some(serde_json::from_value(value)?)),
        }
    }

    pub fn set<V: Serialize>(&self, value: V) -> Result {
        let value = serde_json::to_value(value)?;
        self.driver.borrow_mut().set(self.key.as_str(), value)
    }

    pub fn del(&self, key: &str) -> Result {
        self.driver.borrow_mut().del(key)
    }
}
