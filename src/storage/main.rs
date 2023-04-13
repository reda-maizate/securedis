use dashmap::DashMap;
use log::debug;

use crate::errors::CommandError;

#[derive(Debug, Clone)]
pub struct Storage {
    pub objects: DashMap<String, Record>,
}

#[derive(Debug, Clone)]
pub struct Record {
    pub value: String,
    pub expiration: Option<u64>,
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
            Some(record) => match record.expiration.is_some() {
                true => {
                    let ts_now = chrono::Utc::now().timestamp();
                    debug!(
                        "record expiration: {} / ts_now: {}",
                        record.expiration.unwrap(),
                        ts_now
                    );
                    if record.expiration.unwrap() < ts_now as u64 {
                        return Err(CommandError::InvalidCommand {
                            message: format!("Key {} not found", key),
                        });
                    }
                    Ok(record.value.to_string())
                }
                false => Ok(record.value.to_string()),
            },
            None => Err(CommandError::InvalidCommand {
                message: format!("Key {} not found", key),
            }),
        }
    }

    pub fn set(
        &mut self,
        key: &str,
        value: &str,
        expiration: Option<u64>,
    ) -> Result<(), CommandError> {
        self.objects.insert(
            key.to_string(),
            Record {
                value: value.to_string(),
                expiration,
            },
        );
        Ok(())
    }
}
