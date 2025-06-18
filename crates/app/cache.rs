use std::path::PathBuf;
use std::str::FromStr;

type JsonValue = serde_json::Value;

pub struct FilesystemDriver {
    root: PathBuf,
}

impl FilesystemDriver {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
}

impl sdk::CacheDriver for FilesystemDriver {
    fn get(&self, key: &str) -> sdk::Result<Option<JsonValue>> {
        let file = self.root.join(key);
        if !file.exists() {
            return Ok(None);
        }
        let data = std::fs::read_to_string(file)?;
        let value = JsonValue::from_str(&data)?;
        Ok(Some(value))
    }

    fn set(&mut self, key: &str, value: JsonValue) -> sdk::Result {
        let file = self.root.join(key);
        if !self.root.exists() {
            std::fs::create_dir_all(&self.root)?;
        }
        let json = serde_json::to_value(value)?;
        let data = serde_json::to_string_pretty(&json)?;
        std::fs::write(file, data)?;
        Ok(())
    }

    fn del(&mut self, key: &str) -> sdk::Result {
        let result = std::fs::remove_file(self.root.join(key));
        if let Err(e) = result {
            return if e.kind() != std::io::ErrorKind::NotFound {
                Ok(())
            } else {
                Err(e.into())
            };
        }
        Ok(())
    }
}
