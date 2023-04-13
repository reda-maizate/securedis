use lazy_static::lazy_static;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::OpenOptions;

use crate::structs::CommandError;

lazy_static! {
    static ref STORAGE_PATH: String =
        env::var("STORAGE_PATH").unwrap_or_else(|_| "./src/data.csv".to_string());
}

pub struct Storage {
    pub objects: HashMap<String, String>,
}

enum FileError {
    NotFound { file_name: String },
}

impl Display for FileError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            FileError::NotFound { file_name } => {
                write!(f, "File {} not found in the current directory", file_name)
            }
        }
    }
}

impl Storage {
    pub fn new() -> Storage {
        let mut s = Storage {
            objects: HashMap::new(),
        };
        let _ = s.retrieve_objects_from_csv_file().is_ok();
        s
    }

    fn check_file_exists(&self) -> bool {
        match std::path::Path::new(STORAGE_PATH.as_str()).exists() {
            true => true,
            false => std::fs::File::create(STORAGE_PATH.as_str()).is_ok(),
        }
    }

    fn retrieve_objects_from_csv_file(&mut self) -> Result<(), FileError> {
        // TODO: Check if you need to fix this
        let mut objects: HashMap<String, String> = HashMap::new();
        let is_file_exists = self.check_file_exists();
        match is_file_exists {
            true => {
                let mut reader = csv::Reader::from_path(STORAGE_PATH.as_str()).unwrap();
                for result in reader.records() {
                    let record = result.unwrap();
                    let key = record.get(0).unwrap();
                    let value = record.get(1).unwrap();
                    objects.insert(key.to_string(), value.to_string());
                }
                self.objects = objects;
                Ok(())
            }
            false => Err(FileError::NotFound {
                file_name: STORAGE_PATH.to_string(),
            }),
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

    fn insert_object_into_csv_file(&self, key: &str, value: &str) {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(STORAGE_PATH.as_str())
            .unwrap();

        let mut writer = csv::Writer::from_writer(file);
        writer.write_record([key, value]).unwrap();
    }

    pub fn set(&mut self, key: &str, value: &str) -> Result<(), CommandError> {
        // TODO: Fix this
        // self.objects.insert(key.to_string(), value.to_string());
        // *self.objects.entry(key.to_string()).or_insert(value.to_string()) = value.to_string();
        match self.objects.contains_key(key) {
            true => self.update_object_into_csv_file(key, value),
            false => self.insert_object_into_csv_file(key, value),
        }
        // self.insert_object_into_csv_file(key, value);
        Ok(())
    }

    fn update_object_into_csv_file(&self, key: &str, value: &str) {
        // TODO: Fix this
        let file = OpenOptions::new()
            .write(true)
            .append(false)
            .open(STORAGE_PATH.as_str())
            .unwrap();

        let mut reader = csv::Reader::from_path(STORAGE_PATH.as_str()).unwrap();
        let mut writer = csv::Writer::from_writer(file);
        for result in reader.records() {
            let record = result.unwrap();
            let key_csv = record.get(0).unwrap();
            let value_csv = record.get(1).unwrap();
            if key_csv == key {
                // writer.write_record([key, value]).unwrap();
                println!(
                    "paire a modifier (nouvelle paire): {}-{} / ancienne paire: {}-{}",
                    key, value, key_csv, value_csv
                );
                writer.write_record([key, value]).unwrap();
            } else {
                println!("paire deja existante : {}-{}", key_csv, value_csv);
                writer.write_record([key_csv, value_csv]).unwrap();
            }
        }
    }
}
