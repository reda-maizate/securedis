use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use lazy_static::lazy_static;
use log::{debug, error};

use crate::storage::Storage;
use crate::structs::{CommandError, RESPElement, RESPObject};
use crate::structs::{ECHO_COMMAND, GET_COMMAND, PING_COMMAND, SET_COMMAND};

lazy_static! {
    static ref STORAGE_PATH: String = env::var("STORAGE_PATH").unwrap();
}

pub fn read_next_line(reader: &mut BufReader<TcpStream>, mut input: &mut String) -> String {
    input.clear();
    reader
        .read_line(&mut input)
        .unwrap_or(0);

    // debug!("Next line: {:?}", nxt);
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
        if let Some(_element) = element {
            match _element.content {
                Some(_content) => {
                    // Add a space between each element
                    contents.push_str(&_content);
                    contents.push_str(" ");
                }
                None => {}
            }
        }
    }
    contents.trim().to_string()
}

pub fn process_commands(all_contents: String) -> Result<Option<String>, CommandError> {
    let mut commands: Vec<&str> = all_contents.split(' ').collect();
    debug!("Commands: {:?}", commands);
    let maybe_lowercase_command = commands.remove(0).to_uppercase();
    let command: &str = maybe_lowercase_command.as_str();

    match command {
        ECHO_COMMAND => {
            let contains_argument = check_expected_num_args(commands.clone(), 1);
            match contains_argument {
                Ok(_) => {
                    let message = commands.remove(0);
                    let len_char_msg = message.len().to_string();
                    Ok(Some(format!("${}\r\n{}\r\n", len_char_msg, message)))
                }
                Err(e) => Err(CommandError::InvalidNumberOfArguments { message: "Invalid number of arguments".to_string() })
            }
        }
        SET_COMMAND => {
            let contains_arguments = check_expected_num_args(commands.clone(), 2);
            match contains_arguments {
                Ok(_) => {
                    let key = commands.remove(0);
                    let value = commands.remove(0);
                    let mut storage = Storage::new();

                    match storage.set(key, value) {
                        Ok(_) => Ok(Some("+OK\r\n".to_string())),
                        Err(e) => {
                            error!("Error SET: {:?}", e);
                            Err(CommandError::InvalidCommand { message: format!("Error during insertion of the key-value {}: {}", key, value) })
                        }
                    }
                }
                Err(_e) => Err(CommandError::InvalidNumberOfArguments { message: "Invalid number of arguments".to_string() })
            }
        }
        GET_COMMAND => {
            let contains_arguments = check_expected_num_args(commands.clone(), 1);
            match contains_arguments {
                Ok(_) => {
                    let key = commands.remove(0);
                    let mut storage = Storage::new();

                    match storage.get(key) {
                        Ok(value) => {
                            let len_char_msg = value.len().to_string();
                            Ok(Some(format!("${}\r\n{}\r\n", len_char_msg, value)))
                        }
                        Err(_e) => {
                            error!("Error GET: {:?}", _e);
                            Err(CommandError::InvalidCommand {
                                    message: format!("Key {} not found", key)
                                }
                            )
                        }
                    }
                }
                Err(_e) => Err(CommandError::InvalidNumberOfArguments {
                        message: "Invalid number of arguments".to_string()
                    }
                )
            }
        }
        PING_COMMAND => {
            Ok(Some("+PONG\r\n".to_string()))
        }
        _ => {
            Err(CommandError::InvalidCommand { message: "Invalid command".to_string() })
        }
    }
}

pub fn send_response(mut stream: TcpStream, raw_response: Result<Option<String>, CommandError>) {
    let response = serialize_response(raw_response);
    stream.write_all(response.as_bytes()).unwrap_or(());
}

pub fn serialize_response(response: Result<Option<String>, CommandError>) -> String {
    match response {
        Ok(Some(_response)) => {
            _response
        }
        Ok(None) => {
            "".to_string()
        }
        Err(err) => {
            format!("-ERR {}\r\n", err)
        }
    }
}

pub fn check_expected_num_args(commands: Vec<&str>, expected_num_of_args: usize) -> Result<(), CommandError> {
    if commands.len() < expected_num_of_args {
        Err(CommandError::InvalidNumberOfArguments {
                message: "Invalid number of arguments".to_string()
            }
        )
    } else {
        Ok(())
    }
}