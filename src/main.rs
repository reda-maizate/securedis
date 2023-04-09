mod structs;
mod utils;

use std::io::{BufReader};
use std::net::{TcpListener, TcpStream};
use utils::{read, read_next_line};
use structs::{RESPObject, RESPElement, RESPHeader};
use structs::{RESP_ARRAY_SYMBOL,
              RESP_BULK_STRING_SYMBOL,
              RESP_INTEGER_SYMBOL,
              RESP_ERROR_SYMBOL,
              RESP_SIMPLE_STRING_SYMBOL};
use crate::structs::RESPHeaderType;
use crate::utils::{concatenate_contents, get_last_element, process_commands};


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

fn process_request(mut _request: RESPObject) -> () {
    /* TODO:
     1. Concatenate all the content of the elements
     2. Check for specific commands
     3a. If command is not found, return an error
     3b. If command is found, process the command
     4. Return the result
    */
    let all_contents = concatenate_contents(_request);
    // println!("{}", all_contents);
    process_commands(all_contents);
}


fn handle_connection(stream: TcpStream) -> () {
    let mut reader = BufReader::new(stream);
    let mut input = String::new();

    let mut raw_header = read_next_line(&mut reader, &mut input);

    // Parse and print the first type and number of elements
    let mut resp_object = RESPObject::new();
    let first_resp_element = read_header(&mut raw_header);
    resp_object.elements.push(Some(first_resp_element.clone()));

    // Read the elements and headers of next lines
    for _ in 0..first_resp_element.header.num_of_elements.unwrap() {
        let mut new_parsed_line = read_next_line(&mut reader, &mut input);
        read_header_or_element(&mut new_parsed_line, &mut resp_object);
    }

    process_request(resp_object.clone());
    // println!("{:#?}", resp_object);
}

fn main() {
    println!("Reda's redis server started...");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                handle_connection(_stream)
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}