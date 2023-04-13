use std::env;
use std::fs::OpenOptions;

use dashmap::DashMap;
use lazy_static::lazy_static;
use log::debug;

use crate::errors::FileError;
use crate::storage::main::{Record, Storage};

lazy_static! {
    pub static ref STORAGE_PATH: String =
        env::var("STORAGE_PATH").unwrap_or_else(|_| "./src/data.csv".to_string());
}

#[allow(dead_code)]
pub fn save(storage: Storage, path: &str) {
    for (key, record) in storage.objects.into_iter() {
        insert_object_into_csv_file(key, record.value, Some(path));
    }
}

#[allow(dead_code)]
pub fn load(storage: Storage) -> Result<Storage, FileError> {
    retrieve_objects_from_csv_file(storage.clone())?;
    Ok(storage)
}

#[allow(dead_code)]
fn check_file_exists() -> bool {
    match std::path::Path::new(STORAGE_PATH.as_str()).exists() {
        true => true,
        false => std::fs::File::create(STORAGE_PATH.as_str()).is_ok(),
    }
}

#[allow(dead_code)]
fn retrieve_objects_from_csv_file(mut storage: Storage) -> Result<(), FileError> {
    // TODO: Fix this
    let objects = DashMap::new();
    let is_file_exists = check_file_exists();
    match is_file_exists {
        true => {
            let mut reader = csv::Reader::from_path(STORAGE_PATH.as_str()).unwrap();
            for result in reader.records() {
                let record = result.unwrap();
                let key = record.get(0).unwrap();
                let value = record.get(1).unwrap();
                let expiration = record.get(2).unwrap();
                let record_obj: Record = Record {
                    value: value.to_string(),
                    expiration: Option::from(expiration.parse::<u64>().unwrap()),
                };
                *objects.entry(key.to_string()).or_insert(record_obj) = record_obj.clone();
            }
            storage.objects = objects;
            Ok(())
        }
        false => Err(FileError::NotFound {
            file_name: STORAGE_PATH.to_string(),
        }),
    }
}

fn insert_object_into_csv_file(key: String, value: String, path: Option<&str>) {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(match path {
            Some(p) => p,
            None => STORAGE_PATH.as_str(),
        })
        .unwrap();

    let mut writer = csv::Writer::from_writer(file);
    writer.write_record([key, value]).unwrap();
}

#[allow(dead_code)]
fn update_object_into_csv_file(key: &str, value: &str) {
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
            debug!(
                "paire a modifier (nouvelle paire): {}-{} / ancienne paire: {}-{}",
                key, value, key_csv, value_csv
            );
            writer.write_record([key, value]).unwrap();
        } else {
            debug!("paire deja existante : {}-{}", key_csv, value_csv);
            writer.write_record([key_csv, value_csv]).unwrap();
        }
    }
}
