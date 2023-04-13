use log::error;

use crate::structs::{RESPElement, RESPHeader, RESPHeaderType, RESPObject};
use crate::structs::{
    RESP_ARRAY_SYMBOL, RESP_BULK_STRING_SYMBOL, RESP_ERROR_SYMBOL, RESP_INTEGER_SYMBOL,
    RESP_SIMPLE_STRING_SYMBOL,
};

pub fn read(input: &mut str) -> Vec<char> {
    let mut raw_chars: Vec<char> = input.chars().collect();
    // Remove the two last elements of the vector: \r and \n
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

pub fn read_header(input: &mut str) -> RESPElement {
    let cleaned_chars = read(input);
    let header: RESPHeader = cleaned_chars.into();
    header.into()
}

pub fn read_header_or_element(input: &mut str, resp_object: &mut RESPObject) {
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
        _ => error!("Unknown type of data"),
    }
}
