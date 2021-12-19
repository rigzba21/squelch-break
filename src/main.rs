// CoAP RFC 7252 3. Message Format
// https://datatracker.ietf.org/doc/html/rfc7252#section-3

//quiet warnings for unused code - for now...
#![allow(dead_code)]

use bytes::{Bytes, BytesMut, Buf, BufMut};
use std::error::Error;
use std::net::SocketAddr;
use std::{env, io};
use tokio::net::UdpSocket;

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

struct Server {
    socket: UdpSocket,
    buf: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
}

impl Server {
    async fn run(self) -> Result<(), io::Error> {
        let Server {
            socket,
            mut buf,
            mut to_send,
        } = self;

        loop {
            // First we check to see if there's a message we need to echo back.
            // If so then we try to send it back to the original source, waiting
            // until it's writable and we're able to do so.
            if let Some((size, peer)) = to_send {
                let amt = socket.send_to(&buf[..size], &peer).await?;

                println!("Echoed {}/{} bytes to {}", amt, size, peer);
            }

            // If we're here then `to_send` is `None`, so we take a look for the
            // next message we're going to echo back.
            to_send = Some(socket.recv_from(&mut buf).await?);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let socket = UdpSocket::bind(&addr).await?;
    println!("Listening on: {}", socket.local_addr()?);

    let server = Server {
        socket,
        buf: vec![0; 1024],
        to_send: None,
    };

    // This starts the server task.
    server.run().await?;

    Ok(())
}
