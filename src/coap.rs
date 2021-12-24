//quiet warnings for unused code for now
#![allow(dead_code)]

use bytes::{Bytes};

// CoAP RFC 7252 3. Message Format
// https://datatracker.ietf.org/doc/html/rfc7252#section-3

#[derive(Debug)]
pub struct Message {
    pub version: u8,
    pub msg_type: u8,
    pub token_length: usize,
    pub code: u8,
    pub message_id: u16,
    pub token: Token,
    pub options: Options,
    pub payload: Bytes,
}

#[derive(Debug)]
pub struct Token {
    pub bytes: Bytes,
}

#[derive(Debug)]
pub struct Options {
    pub option_delta: u8,
    pub option_length: u8,
    pub option_val: u8,
}

#[derive(Debug)]
pub enum OptionValue {
    Empty,
    Opaque,
    Uint,
    OptString,
}
