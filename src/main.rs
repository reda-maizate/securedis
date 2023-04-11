mod structs;
mod utils;

extern crate env_logger;
extern crate log;

use env_logger::Builder;
use log::{debug, error, info, warn};
use chrono::Utc;
use std::io::{BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use utils::{read, read_next_line};
use structs::{RESPObject, RESPElement, RESPHeader};
use structs::{RESP_ARRAY_SYMBOL,
              RESP_BULK_STRING_SYMBOL,
              RESP_INTEGER_SYMBOL,
              RESP_ERROR_SYMBOL,
              RESP_SIMPLE_STRING_SYMBOL};
use crate::structs::{CommandError, RESPHeaderType};
use crate::utils::{concatenate_contents, get_last_element, process_commands, send_response};

fn configure_logger() {
    let mut builder = Builder::from_env("LOGLEVEL");
    builder.init();
}


fn read_header(input: &mut String) -> RESPElement {
    let cleaned_chars = read(input);
    let header: RESPHeader = cleaned_chars.into();
    header.into()
}

fn read_header_or_element(input: &mut String, resp_object: &mut RESPObject) {
    let cleaned_chars = read(input);

    // Check if this line is a header or an element
    match cleaned_chars[0] {
        RESP_ARRAY_SYMBOL => {
            let header: RESPHeader = cleaned_chars.into();
            let element: RESPElement = header.into();
            resp_object.elements.push(Some(element));
        }
        RESP_BULK_STRING_SYMBOL => {
            let header: RESPHeader = cleaned_chars.into();
            let element: RESPElement = header.into();
            resp_object.elements.push(Some(element));
        }
        RESP_INTEGER_SYMBOL => {
            let header: RESPHeader = cleaned_chars.into();
            let element: RESPElement = header.into();
            resp_object.elements.push(Some(element));
        }
        RESP_ERROR_SYMBOL => {
            let header: RESPHeader = cleaned_chars.into();
            let element: RESPElement = header.into();
            resp_object.elements.push(Some(element));
        }
        RESP_SIMPLE_STRING_SYMBOL => {
            let header: RESPHeader = cleaned_chars.into();
            let element: RESPElement = header.into();
            resp_object.elements.push(Some(element));
        }
        // This is an element
        'A'..='Z' | 'a'..='z' => {
            let mut last_element = get_last_element(resp_object).unwrap();
            let chars_to_str: String = cleaned_chars.into_iter().collect();
            last_element.content = Some(chars_to_str);
        }
        '0'..='9' => {
            let mut last_element = get_last_element(resp_object).unwrap();
            let chars_to_str: String = cleaned_chars.into_iter().collect();
            last_element.header.resp_type = Some(RESPHeaderType::Integer);
            last_element.content = Some(chars_to_str);
        }
        _ => panic!("Unknown type of data"),
    }
}

fn process_request(mut _request: RESPObject) -> Result<Option<String>, CommandError> {
    let all_contents = concatenate_contents(_request);
    debug!("All contents: {:?}", all_contents);
    process_commands(all_contents)
}


fn handle_connection(mut stream: TcpStream) -> (TcpStream, Result<Option<String>, CommandError>) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut input = String::new();

    let mut raw_header = read_next_line(&mut reader, &mut input);
    match raw_header.is_empty() {
            false => {  }
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

    let output: Result<Option<String>, CommandError> = process_request(resp_object.clone());
    (stream, output)
}


fn main() {
    configure_logger();
    info!("Reda's server is now started...");
    let listener = Arc::new(Mutex::new(TcpListener::bind("127.0.0.1:6379").unwrap()));

    for stream in listener.lock().unwrap().incoming() {
        thread::spawn(move|| {
            match stream {
                Ok(_stream) => loop {
                    let (stream, output) = handle_connection(_stream.try_clone().unwrap());
                    send_response(stream, output);
                },
                Err(e) => {
                    error!("error: {}", e);
                }
            }
        });
    }
}