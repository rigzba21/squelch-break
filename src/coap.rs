//quiet warnings for unused code for now
#![allow(dead_code)]

use bytes::{Bytes};

// CoAP RFC 7252 3. Message Format
// https://datatracker.ietf.org/doc/html/rfc7252#section-3
pub struct Message {
    pub version: u8,
    pub msg_type: u8,
    pub token_length: u8,
    pub code: Bytes,
    pub message_id: Bytes,
    pub token: Token,
    pub options: Options,
    pub payload: Bytes,
}

pub struct Token {
    pub bytes: Bytes,
}

pub struct Options {
    pub option_delta: Bytes,
    pub option_length: Bytes,
    pub option_val: OptionValue,
}

pub enum OptionValue {
    Empty,
    Opaque,
    Uint,
    OptString,
}
