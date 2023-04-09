use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use crate::structs::{CommandError, RESPElement, RESPObject};
use crate::structs::{PING_COMMAND, GET_COMMAND, SET_COMMAND, ECHO_COMMAND};

pub fn read_next_line(reader: &mut BufReader<TcpStream>, mut input: &mut String) -> String {
    input.clear();
    reader
        .read_line(&mut input)
        .unwrap();

    input.to_string()
}

pub fn read(input: &mut String) -> Vec<char> {
    let mut raw_chars: Vec<char> = input
        .chars()
        .collect();

    // Remove the two last elements of the vector: \n and \r
    raw_chars.pop();
    raw_chars.pop();
    raw_chars
}

pub fn get_last_element(resp_object: &mut RESPObject) -> Option<&mut RESPElement> {
    match resp_object.elements.len() {
        0 => None,
        n => resp_object.elements[n - 1].as_mut(),
    }
}

pub fn concatenate_contents(resp_object: RESPObject) -> String {
    let mut contents: String = String::new();

    for element in resp_object.elements {
        match element {
            Some(_element) => {
                match _element.content {
                    Some(_content) => {
                        // Add a space between each element
                        contents.push_str(&_content);
                        contents.push_str(" ");
                    }
                    None => {}
                }
            }
            None => {}
        }
    }
    contents.trim().to_string()
}

pub fn process_commands(all_contents: String) -> Result<String, CommandError> {
    let mut commands: Vec<&str> = all_contents.split(" ").collect();
    let command = commands.remove(0);

    match command {
        ECHO_COMMAND => {
            let message = commands.remove(0);
            // TODO - Error handling:
            // If message is out of bounds, return a CommandError::InvalidNumberOfArguments
            // If everything is good, return a String with the message: "ECHO <message>"
            Ok(format!("ECHO {}", message))
        }
        SET_COMMAND => {
            let key = commands.remove(0);
            let value = commands.remove(0);
            // TODO - Error handling:
            // If key or value is out of bounds, return a CommandError::InvalidNumberOfArguments
            // println!("SET {} {}", key, value);
            Ok(format!("SET {} {}", key, value))
        }
        GET_COMMAND => {
            let key = commands.remove(0);
            // println!("GET {}", key);
            Ok(format!("GET {}", key))
        }
        PING_COMMAND => {
            // println!("PONG");
            Ok(format!("PONG"))
        }
        _ => {
            Err(CommandError::InvalidCommand { message: "Invalid command".to_string() })
        }
    }
}

pub fn send_response(mut stream: TcpStream, raw_response: String) {
    let response = serialize_response(raw_response);
    println!("Response: {}", response.clone());
    stream.write(response.as_bytes()).unwrap();
}

pub fn serialize_response(response: String) -> String {
    let mut serialized_response: String = String::new();
    serialized_response.push_str("+");
    serialized_response.push_str(&response);
    serialized_response.push_str("\r\n");
    serialized_response
}