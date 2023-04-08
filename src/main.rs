use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};


#[derive(Debug, Copy, Clone)]
struct RESPHeader {
    resp_type: Option<RESPHeaderType>,
    num_of_elements: Option<i32>,
}

#[derive(Debug)]
struct RESPObject {
    elements: Option<RESPElement>,
}

#[derive(Debug, Clone)]
struct RESPElement {
    header: RESPHeader,
    content: Option<String>,
    next: Box<Option<RESPElement>>,
}

#[derive(Debug, Copy, Clone)]
enum RESPHeaderType {
    Array,
    BulkString,
    Integer,
    Error,
    SimpleString,
}

#[derive(Debug)]
enum Either<A, B> {
    Left(A),
    Right(B),
}

impl RESPObject {
    fn new() -> RESPObject {
        RESPObject {
            elements: None,
        }
    }
}

impl RESPElement {
    fn new() -> RESPElement {
        RESPElement {
            header: RESPHeader {
                resp_type: None,
                num_of_elements: None,
            },
            content: None,
            next: Box::new(None),
        }
    }
}

impl From<Vec<char>> for RESPElement {
    fn from(str: Vec<char>) -> RESPElement {
        let content: String = str.into_iter().collect();
        RESPElement {
            header: RESPHeader { resp_type: None, num_of_elements: None },
            content: Some(content),
            next: Box::new(None),
        }
    }
}

impl From<RESPHeader> for RESPElement {
    fn from(header: RESPHeader) -> RESPElement {
        RESPElement {
            header: RESPHeader {
                resp_type: header.resp_type,
                num_of_elements: header.num_of_elements
            },
            content: None,
            next: Box::new(None),
        }
    }
}


impl From<char> for RESPHeaderType {
    fn from(c: char) -> RESPHeaderType {
        match c {
            '*' => RESPHeaderType::Array,
            '$' => RESPHeaderType::BulkString,
            ':' => RESPHeaderType::Integer,
            '-' => RESPHeaderType::Error,
            '+' => RESPHeaderType::SimpleString,
            _ => panic!("Invalid type of data"),
        }
    }
}

impl From<Vec<char>> for RESPHeader {
    fn from(s: Vec<char>) -> RESPHeader {
        let resp_type = s[0].into();
        let num_of_elements: i32 = s[1].to_digit(10).unwrap() as i32;

        match resp_type {
            // For Array, the next line is a header and the following lines are elements
            // TODO: Check for the others types
            RESPHeaderType::Array => RESPHeader {
                resp_type: Some(RESPHeaderType::Array),
                num_of_elements: Some((num_of_elements * 2) as i32),
            },
            _ => RESPHeader {
                resp_type: Some(resp_type),
                num_of_elements: Some(num_of_elements),
            }
        }
    }
}

fn read_next_line(reader: &mut BufReader<TcpStream>, mut input: &mut String) -> String {
    input.clear();
    reader
        .read_line(&mut input)
        .unwrap();

    input.to_string()
}

fn read(input: &mut String) -> Vec<char> {
    let mut raw_chars: Vec<char> = input
        .chars()
        .collect();

    // Remove the two last elements of the vector: \n and \r
    raw_chars.pop();
    raw_chars.pop();
    raw_chars
}

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
            let mut element: RESPElement = header.into();
            last_element.next = Box::new(Some(element.clone()));
        }
        '$' => {
            let header: RESPHeader = cleaned_chars.into();
            let mut element: RESPElement = header.into();
            last_element.next = Box::new(Some(element.clone()));
        }
        ':' => {
            let header: RESPHeader = cleaned_chars.into();
            let mut element: RESPElement = header.into();
            last_element.next = Box::new(Some(element.clone()));
        }
        '-' => {
            let header: RESPHeader = cleaned_chars.into();
            let mut element: RESPElement = header.into();
            last_element.next = Box::new(Some(element.clone()));
        }
        '+' => {
            let header: RESPHeader = cleaned_chars.into();
            let mut element: RESPElement = header.into();
            last_element.next = Box::new(Some(element.clone()));
        }
        // This is an element
        'A'..='Z' | 'a'..='z' | '0'..='9' => {
            last_element.content = Some(cleaned_chars.into_iter().collect());
        }
        _ => panic!("Unknown type of data"),
    }
}


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