use crate::errors::CommandError;
use dashmap::DashMap;
use log::debug;

#[derive(Debug, Clone)]
pub struct Storage {
    pub objects: DashMap<String, String>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            objects: DashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Result<String, CommandError> {
        debug!("{:?}", self.objects);
        match self.objects.get(key) {
            Some(value) => Ok(value.to_string()),
            None => Err(CommandError::InvalidCommand {
                message: format!("Key {} not found", key),
            }),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<(), CommandError> {
        self.objects.insert(key.to_string(), value.to_string());
        Ok(())
    }
}
