//quiet warnings for unused code for now
#![allow(dead_code)]

use bytes::{Bytes};

use crate::cbor::{placeholder};


// CoAP RFC 7252 3. Message Format
// https://datatracker.ietf.org/doc/html/rfc7252#section-3
pub struct Message {
    version: Bytes,
    msg_type: Bytes,
    code: Bytes,
    message_id: Bytes,
    token: Token,
    options: Options,
    payload: Bytes,
}

struct Token {
    length: Bytes,
    bytes: Bytes,
}

struct Options {
    option_delta: Bytes,
    option_length: Bytes,
    option_val: OptionValue,
}

enum OptionValue {
    Empty,
    Opaque,
    Uint,
    OptString,
}


