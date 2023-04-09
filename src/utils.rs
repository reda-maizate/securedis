use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use crate::structs::{RESPElement, RESPObject};

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

// TODO: The function below get the last element of the linked list resp_object.elements
pub fn get_last_element(resp_object: &mut RESPObject) -> Option<RESPElement> {
    let mut last_element = resp_object.elements.clone();

    while last_element.next.is_some() {
        last_element = Some(last_element.next)?;
    }
    last_element.clone()
}
