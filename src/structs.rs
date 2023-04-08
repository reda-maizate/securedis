#[derive(Debug, Copy, Clone)]
pub struct RESPHeader {
    pub resp_type: Option<RESPHeaderType>,
    pub num_of_elements: Option<i32>,
}

#[derive(Debug)]
pub struct RESPObject {
    pub elements: Option<RESPElement>,
}

#[derive(Debug, Clone)]
pub struct RESPElement {
    pub header: RESPHeader,
    pub content: Option<String>,
    pub next: Box<Option<RESPElement>>,
}

#[derive(Debug, Copy, Clone)]
pub enum RESPHeaderType {
    Array,
    BulkString,
    Integer,
    Error,
    SimpleString,
}

impl RESPObject {
    pub fn new() -> RESPObject {
        RESPObject {
            elements: None,
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