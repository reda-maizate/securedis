use std::io::{BufRead, BufReader};
use std::net::TcpStream;

use crate::structs::{CommandError, RESPElement, RESPObject};


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

pub fn process_commands(all_contents: String) -> Result<(), CommandError>{
    let mut commands: Vec<&str> = all_contents.split(" ").collect();
    let command = commands.remove(0);

    match command {
        "ECHO" => {
            let words = commands.remove(0);
            println!("ECHO {}", words);
            Ok(())
        }
        "SET" => {
            let key = commands.remove(0);
            let value = commands.remove(0);
            println!("SET {} {}", key, value);
            Ok(())
        }
        "GET" => {
            let key = commands.remove(0);
            println!("GET {}", key);
            Ok(())
        }
        "DEL" => {
            let key = commands.remove(0);
            println!("DEL {}", key);
            Ok(())
        }
        "EXISTS" => {
            let key = commands.remove(0);
            println!("EXISTS {}", key);
            Ok(())
        }
        "EXPIRE" => {
            let key = commands.remove(0);
            let seconds = commands.remove(0);
            println!("EXPIRE {} {}", key, seconds);
            Ok(())
        }
        "TTL" => {
            let key = commands.remove(0);
            println!("TTL {}", key);
            Ok(())
        }
        "KEYS" => {
            let pattern = commands.remove(0);
            println!("KEYS {}", pattern);
            Ok(())
        }
        "FLUSHDB" => {
            println!("FLUSHDB");
            Ok(())
        }
        "FLUSHALL" => {
            println!("FLUSHALL");
            Ok(())
        }
        "DBSIZE" => {
            println!("DBSIZE");
            Ok(())
        }
        "PING" => {
            println!("PONG");
            Ok(())
        }
        "QUIT" => {
            println!("QUIT");
            Ok(())
        }
        _ => {
            println!("Command not found");
            Ok(())
        }
    }
}