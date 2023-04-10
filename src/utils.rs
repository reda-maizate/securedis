use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use crate::structs::{CommandError, RESPElement, RESPObject};
use crate::structs::{PING_COMMAND, GET_COMMAND, SET_COMMAND, ECHO_COMMAND};

pub fn read_next_line(reader: &mut BufReader<TcpStream>, mut input: &mut String) -> String {
    input.clear();
    reader
        .read_line(&mut input)
        .unwrap_or(0);

    // println!("Next line: {:?}", nxt);
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

pub fn process_commands(all_contents: String) -> Option<String> {
    let mut commands: Vec<&str> = all_contents.split(" ").collect();
    // println!("Commands: {:?}", commands);
    let maybe_lowercase_command = commands.remove(0).to_uppercase();
    let command: &str = maybe_lowercase_command.as_str();

    match command {
        ECHO_COMMAND => {
            let message = commands.remove(0);
            // TODO - Error handling:
            // If message is out of bounds, return a CommandError::InvalidNumberOfArguments
            // If everything is good, return a String with the message: "ECHO <message>"
            Some(format!("ECHO {}", message))
        }
        // SET_COMMAND => {
        //     let key = commands.remove(0);
        //     let value = commands.remove(0);
        //     // TODO - Error handling:
        //     // If key or value is out of bounds, return a CommandError::InvalidNumberOfArguments
        //     // println!("SET {} {}", key, value);
        //     Ok(format!("SET {} {}", key, value))
        // }
        // GET_COMMAND => {
        //     let key = commands.remove(0);
        //     // println!("GET {}", key);
        //     Ok(format!("GET {}", key))
        // }
        PING_COMMAND => {
            // println!("returning: PONG");
            Some(format!("PONG"))
        }
        _ => {
            // TODO: In reality, here sending an error shut down the program. That's not what we want.
            // We want to send an error to the client and keep the server running.
            // Err(CommandError::InvalidCommand { message: "Invalid command".to_string() })
            // Ok((String::from("")))
            None
        }
    }
}

pub fn send_response(mut stream: TcpStream, raw_response: Option<String>) {
    let response = serialize_response(raw_response);
    // println!("Response: {}", response.clone());
    stream.write(response.as_bytes()).unwrap_or(0);
}

pub fn serialize_response(response: Option<String>) -> String {
    match response {
        Some(_response) => {
            let mut serialized_response: String = String::new();
            serialized_response.push_str("+");
            serialized_response.push_str(&_response);
            serialized_response.push_str("\r\n");
            serialized_response
        }
        None => {
            let mut serialized_response: String = String::new();
            serialized_response.push_str("-ERR unknown command\r\n");
            serialized_response
        }
    }
    // let mut serialized_response: String = String::new();
    // serialized_response.push_str("+");
    // serialized_response.push_str(&response);
    // serialized_response.push_str("\r\n");
    // serialized_response
}