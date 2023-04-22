extern crate env_logger;
extern crate log;

use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

use env_logger::Builder;
use log::{debug, error, info};

use structs::RESPObject;
use utils::read_next_line;

use crate::deserialize::{read_header, read_header_or_element};
use crate::errors::CommandError;
use crate::storage::main::Storage;
use crate::utils::{concatenate_contents, process_commands, send_response};

mod deserialize;
mod errors;
mod process;
mod storage;
mod structs;
mod tests;
mod utils;

fn configure_logger() {
    let mut builder = Builder::from_default_env();
    builder.init();
}

fn process_request(
    mut _request: RESPObject,
    storage: MutexGuard<Storage>,
) -> Result<Option<String>, CommandError> {
    let all_contents = concatenate_contents(_request);
    debug!("All contents: {:?}", all_contents);
    process_commands(all_contents, storage)
}

fn handle_connection(
    stream: TcpStream,
    storage: MutexGuard<Storage>,
) -> (TcpStream, Result<Option<String>, CommandError>) {
    // debug!("New connection: {:?}", stream);
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut input = String::new();

    let mut raw_header = read_next_line(&mut reader, &mut input);
    match raw_header.is_empty() {
        false => {}
        true => {
            return (stream, Ok(None));
        }
    }
    // Parse and print the first type and number of elements
    let mut resp_object = RESPObject::new();
    let first_resp_element = read_header(&mut raw_header);
    resp_object.elements.push(Some(first_resp_element.clone()));

    // Read the elements and headers of next lines
    for _ in 0..first_resp_element.header.num_of_elements.unwrap() {
        let mut new_parsed_line = read_next_line(&mut reader, &mut input);
        read_header_or_element(&mut new_parsed_line, &mut resp_object);
    }

    let output: Result<Option<String>, CommandError> =
        process_request(resp_object.clone(), storage);
    (stream, output)
}

fn main() {
    configure_logger();
    info!("Securedis server is now started...");
    let listener = Arc::new(Mutex::new(TcpListener::bind("127.0.0.1:6379").unwrap()));
    let storage = Arc::new(Mutex::new(Storage::new()));

    for stream in listener.lock().unwrap().incoming() {
        let shared_storage = storage.clone();
        thread::spawn(move || match stream {
            Ok(_stream) => loop {
                let (stream, output) =
                    handle_connection(_stream.try_clone().unwrap(), shared_storage.lock().unwrap());
                send_response(stream, output);
            },
            Err(_e) => {
                error!("error: {}", _e);
            }
        });
    }
}
