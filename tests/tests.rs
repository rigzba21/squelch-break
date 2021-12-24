use bytes::{Bytes};

use squelch_break::{Token, Message};

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
    


    /*let message = Message {
        version: 0x01,
        msg_type: 0x00,
        token_length: 
    }*/
}

