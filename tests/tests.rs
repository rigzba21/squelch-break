use bytes::{Bytes};

use squelch_break::{Token, Message, Options};

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn test_version() {
    let version: u8 = 0b01;
    println!("{:#?}", format!("{:#b}", version));
    assert_eq!(version, 0x01);
}

#[test]
fn test_sample_message() {
    let token_bytes = Bytes::from("test_token");
    
    let token = Token {
        bytes: token_bytes,
    };
    
    let token_length = token.bytes.len();

    let payload = Bytes::from("Some dummy payload");

    let options = Options {
        option_delta: 0x010,
        option_length: 0xab,
        option_val: 0x00, //empty  options
    };
    
    let message = Message {
        version: 0x01,
        msg_type: 0x00,
        token_length: token_length,
        code: 0x00000011,
        message_id: 0x0000000000000001,
        token: token,
        options: options,
        payload: payload,
    };

    println!("{:#?}", message);
}

