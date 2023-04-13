use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use crate::errors::CommandError;
use crate::process::{
    process_echo,
    process_get,
    // process_save,
    process_set,
};
use crate::storage::main::Storage;
use lazy_static::lazy_static;
use log::debug;

use crate::structs::{RESPElement, RESPObject};
use crate::structs::{ECHO_COMMAND, GET_COMMAND, PING_COMMAND, SET_COMMAND};

lazy_static! {
    static ref STORAGE_PATH: String =
        env::var("STORAGE_PATH").unwrap_or_else(|_| "./src/data.csv".to_string());
}

pub fn read_next_line(reader: &mut BufReader<TcpStream>, input: &mut String) -> String {
    input.clear();
    reader.read_line(input).unwrap_or(0);

    // debug!("Next line: {:?}", nxt);
    input.to_string()
}

pub fn read(input: &mut str) -> Vec<char> {
    let mut raw_chars: Vec<char> = input.chars().collect();

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

    for element in resp_object.elements.into_iter().flatten() {
        if let Some(_content) = element.content {
            contents.push_str(&_content);
            contents.push(' ');
        }
    }

    contents.trim().to_string()
}

pub fn process_commands(all_contents: String) -> Result<Option<String>, CommandError> {
    let mut commands: Vec<&str> = all_contents.split(' ').collect();
    debug!("Commands: {:?}", commands);
    let maybe_lowercase_command = commands.remove(0).to_uppercase();
    let command: &str = maybe_lowercase_command.as_str();
    let storage = Storage::new();

    match command {
        ECHO_COMMAND => process_echo(commands),
        SET_COMMAND => process_set(commands, storage),
        GET_COMMAND => process_get(commands, storage),
        // SAVE_COMMAND => process_save(commands, storage),
        PING_COMMAND => Ok(Some("+PONG\r\n".to_string())),
        _ => Err(CommandError::InvalidCommand {
            message: "Invalid command".to_string(),
        }),
    }
}

pub fn send_response(mut stream: TcpStream, raw_response: Result<Option<String>, CommandError>) {
    let response = serialize_response(raw_response);
    stream.write_all(response.as_bytes()).unwrap_or(());
}

pub fn serialize_response(response: Result<Option<String>, CommandError>) -> String {
    match response {
        Ok(Some(_response)) => _response,
        Ok(None) => "".to_string(),
        Err(err) => {
            format!("-ERR {}\r\n", err)
        }
    }
}

pub fn check_expected_num_args(
    commands: Vec<&str>,
    expected_num_of_args: usize,
) -> Result<(), CommandError> {
    if commands.len() < expected_num_of_args {
        Err(CommandError::InvalidNumberOfArguments {
            message: "Invalid number of arguments".to_string(),
        })
    } else {
        Ok(())
    }
}
