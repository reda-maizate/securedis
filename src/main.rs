mod structs;
mod utils;

use std::io::{BufReader};
use std::net::{TcpListener, TcpStream};
use utils::{read, read_next_line};
use structs::{RESPObject, RESPElement, RESPHeader};


fn read_header(input: &mut String) -> RESPElement {
    let cleaned_chars = read(input);
    let header: RESPHeader = cleaned_chars.into();
    header.into()
}

fn read_header_or_element(input: &mut String, resp_object: &mut RESPObject) -> () {
    let cleaned_chars = read(input);
    let last_element = resp_object.elements.as_mut().unwrap();

    // Check if this line is a header or an element
    match cleaned_chars[0] {
        '*' => {
            let header: RESPHeader = cleaned_chars.into();
            let element: RESPElement = header.into();
            last_element.next = Box::new(Some(element.clone()));
        }
        '$' => {
            let header: RESPHeader = cleaned_chars.into();
            let element: RESPElement = header.into();
            last_element.next = Box::new(Some(element.clone()));
        }
        ':' => {
            let header: RESPHeader = cleaned_chars.into();
            let element: RESPElement = header.into();
            last_element.next = Box::new(Some(element.clone()));
        }
        '-' => {
            let header: RESPHeader = cleaned_chars.into();
            let element: RESPElement = header.into();
            last_element.next = Box::new(Some(element.clone()));
        }
        '+' => {
            let header: RESPHeader = cleaned_chars.into();
            let element: RESPElement = header.into();
            last_element.next = Box::new(Some(element.clone()));
        }
        // This is an element
        'A'..='Z' | 'a'..='z' | '0'..='9' => {
            last_element.content = Some(cleaned_chars.into_iter().collect());
        }
        _ => panic!("Unknown type of data"),
    }
}

#[allow(dead_code)]
fn process_request(_request: RESPObject) -> () {
    // TODO: Implement this but first we need to implement the RESP struct
}


fn handle_connection(stream: TcpStream) -> () {
    let mut reader = BufReader::new(stream);
    let mut input = String::new();

    let mut raw_header = read_next_line(&mut reader, &mut input);

    // Parse and print the first type and number of elements
    let mut resp_object = RESPObject::new();
    let first_resp_element = read_header(&mut raw_header);
    resp_object.elements = Some(first_resp_element.clone());

    // Read the elements and headers of next lines
    for _ in 0..first_resp_element.header.num_of_elements.unwrap() {
        let mut new_parsed_line = read_next_line(&mut reader, &mut input);
        read_header_or_element(&mut new_parsed_line, &mut resp_object);
    }
    println!("{:#?}", resp_object);
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