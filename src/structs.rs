pub const RESP_ARRAY_SYMBOL: char = '*';
pub const RESP_BULK_STRING_SYMBOL: char = '$';
pub const RESP_INTEGER_SYMBOL: char = ':';
pub const RESP_ERROR_SYMBOL: char = '-';
pub const RESP_SIMPLE_STRING_SYMBOL: char = '+';

pub const ECHO_COMMAND: &str = "ECHO";
pub const SET_COMMAND: &str = "SET";
pub const GET_COMMAND: &str = "GET";
pub const PING_COMMAND: &str = "PING";
// pub const SAVE_COMMAND: &str = "SAVE";
// pub const LOAD_COMMAND: &str = "LOAD";
pub const EXPIRE_COMMAND: &str = "PX";

pub const OK_RESPONSE: &str = "+OK\r\n";
pub const PONG_RESPONSE: &str = "+PONG\r\n";
pub const NIL_RESPONSE: &str = "$-1\r\n";

#[derive(Debug, Copy, Clone)]
pub struct RESPHeader {
    pub resp_type: Option<RESPHeaderType>,
    pub num_of_elements: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct RESPObject {
    pub elements: Vec<Option<RESPElement>>,
}

#[derive(Debug, Clone)]
pub struct RESPElement {
    pub header: RESPHeader,
    pub content: Option<String>,
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
            elements: Vec::new(),
        }
    }
}

impl From<Vec<char>> for RESPElement {
    fn from(str: Vec<char>) -> RESPElement {
        let content: String = str.into_iter().collect();
        RESPElement {
            header: RESPHeader {
                resp_type: None,
                num_of_elements: None,
            },
            content: Some(content),
        }
    }
}

impl From<RESPHeader> for RESPElement {
    fn from(header: RESPHeader) -> RESPElement {
        RESPElement {
            header: RESPHeader {
                resp_type: header.resp_type,
                num_of_elements: header.num_of_elements,
            },
            content: None,
        }
    }
}

impl From<char> for RESPHeaderType {
    fn from(c: char) -> RESPHeaderType {
        match c {
            RESP_ARRAY_SYMBOL => RESPHeaderType::Array,
            RESP_BULK_STRING_SYMBOL => RESPHeaderType::BulkString,
            RESP_INTEGER_SYMBOL => RESPHeaderType::Integer,
            RESP_ERROR_SYMBOL => RESPHeaderType::Error,
            RESP_SIMPLE_STRING_SYMBOL => RESPHeaderType::SimpleString,
            _ => panic!("Invalid type of data"),
        }
    }
}

impl From<Vec<char>> for RESPHeader {
    fn from(s: Vec<char>) -> RESPHeader {
        let resp_type = s[0].into();
        let num_of_elements: i32 = s[1].to_digit(10).unwrap() as i32;

        match resp_type {
            RESPHeaderType::Array => RESPHeader {
                resp_type: Some(RESPHeaderType::Array),
                num_of_elements: Some(num_of_elements * 2),
            },
            _ => RESPHeader {
                resp_type: Some(resp_type),
                num_of_elements: Some(num_of_elements),
            },
        }
    }
}
