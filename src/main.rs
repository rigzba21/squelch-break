
// CoAP RFC 7252 3. Message Format
// https://datatracker.ietf.org/doc/html/rfc7252#section-3

struct Message {
    version: u8,
    msg_type: u8,
    code: u8,
    message_id: u16,
    token: Token,
    options: Options,
}

struct Token {
    length: u8,
    bytes: [u8; 8],
}

struct Options {
    option_delta: u8,
    option_length: u8,
    //TODO: Option Value
}

fn main() {
    println!("Hello, world!");
}
