//quiet warnings for unused code for now
#![allow(dead_code)]

use bytes::{Bytes};

use crate::cbor::{placeholder};


// CoAP RFC 7252 3. Message Format
// https://datatracker.ietf.org/doc/html/rfc7252#section-3
pub struct Message {
    version: u8,
    msg_type: u8,
    code: u8,
    message_id: u16,
    token: Token,
    options: Options,
    payload: Bytes,
}

struct Token {
    length: u8,
    bytes: Bytes,
}

struct Options {
    option_delta: u8,
    option_length: u8,
    option_val: OptionValue,
}

enum OptionValue {
    Empty,
    Opaque,
    Uint,
    OptString,
}


