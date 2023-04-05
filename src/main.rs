use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

#[derive(Debug)]
struct RESPHeader {
    resp_type: RESPHeaderType,
    num_of_elements: i32,
}

#[derive(Debug)]
struct RESPObject {
    header: Option<RESPHeader>,
    elements: Vec<RESPElement>,
}

#[derive(Debug)]
struct RESPElement {
    // TODO: Ajouter un header pour bien tout binder
    // header: RESPHeader,
    content: String
}

#[derive(Debug)]
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
            header: None,
            elements: Vec::new(),
        }
    }
}

impl From<Vec<char>> for RESPElement {
    fn from(s: Vec<char>) -> RESPElement {
        let content: String = s.into_iter().collect();
        RESPElement {
            content
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
                resp_type,
                num_of_elements: (num_of_elements * 2) as i32,
            },
            _ => RESPHeader {
                resp_type,
                num_of_elements,
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

fn read_header(input: &mut String) -> RESPHeader {
    let mut raw_chars: Vec<char> = input
        .chars()
        .collect();

    // Remove the two last elements of the vector: \n and \r
    raw_chars.pop();
    raw_chars.pop();

    let header: RESPHeader = raw_chars.into();
    header
}

fn read_header_or_element(input: &mut String) -> Either<RESPHeader, RESPElement> {
    let mut raw_chars: Vec<char> = input
        .chars()
        .collect();

    // Remove the two last elements of the vector: \n and \r
    raw_chars.pop();
    raw_chars.pop();

    // Check if this line is a header or an element
    match raw_chars[0] {
        '*' => {
            let header: RESPHeader = raw_chars.into();
            // println!("h: {:?}", header);
            Either::Left(header)
        }
        '$' => {
            let header: RESPHeader = raw_chars.into();
            // println!("h: {:?}", header);
            Either::Left(header)
        }
        ':' => {
            let header: RESPHeader = raw_chars.into();
            // println!("h: {:?}", header);
            Either::Left(header)
        }
        '-' => {
            let header: RESPHeader = raw_chars.into();
            // println!("h: {:?}", header);
            Either::Left(header)
        }
        '+' => {
            let header: RESPHeader = raw_chars.into();
            // println!("h: {:?}", header);
            Either::Left(header)
        }
        // This is an element
        'A'..='Z' | 'a'..='z' | '0'..='9' => {
            let element: RESPElement = raw_chars.into();
            // println!("detected element : {:?}", element);
            Either::Right(element)
        }
        _ => panic!("Unknown type of data"),
    }
}


fn process_request(request: RESPObject) -> () {
    // TODO: Implement this but first we need to implement the RESP struct
}


fn handle_connection(stream: TcpStream) -> () {
    let mut reader = BufReader::new(stream);
    let mut input = String::new();
    let mut resp = RESPObject::new();

    let mut raw_header = read_next_line(&mut reader, &mut input);

    // Parse and print the first type and number of elements
    let header = read_header(&mut raw_header);
    println!("{:?}", header);

    // Read the elements and headers of next lines
    for _ in 0..header.num_of_elements {
        let mut nl = read_next_line(&mut reader, &mut input);
        let nh = read_header_or_element(&mut nl);
        println!("next header/element : {:?}", nh);
    }
}


fn main() {
    println!("Reda's redis server started...");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                handle_connection(_stream)
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
