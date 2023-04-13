use crate::errors::CommandError;
use std::collections::HashMap;

pub struct Storage {
    pub objects: HashMap<String, String>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            objects: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Result<String, CommandError> {
        match self.objects.get(key) {
            Some(value) => Ok(value.to_string()),
            None => Err(CommandError::InvalidCommand {
                message: format!("Key {} not found", key),
            }),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<(), CommandError> {
        // TODO: Fix this
        self.objects.insert(key.to_string(), value.to_string());
        // *self.objects.entry(key.to_string()).or_insert(value.to_string()) = value.to_string();
        // match self.objects.contains_key(key) {
        //     true => self.update_object_into_csv_file(key, value),
        //     false => self.insert_object_into_csv_file(key, value, None),
        // }
        // self.insert_object_into_csv_file(key, value);
        Ok(())
    }
}
