//use std::process;
use std::error::Error;
use std::net::SocketAddr;
use std::{env, io};
use tokio::net::UdpSocket;
use orion::aead;

struct SquelchBreakHandler {
    socket: UdpSocket,
    secretkey: aead::SecretKey,
    buffer: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
}

//TODO: message struct


impl SquelchBreakHandler {
    //TODO: distribute aead::SecretKey to sbrunner(s)

    //run the event loop
    async fn run(self) -> Result<(), io::Error> {
        let SquelchBreakHandler {
            socket,
            secretkey,
            mut buffer,
            mut to_send,
        } = self;

        loop {
            if let Some((size, _sender)) = to_send {
                println!("Received message from: {:#?}", _sender);

                //verify authenticated message and decrypt
                let verify = aead::open(&secretkey, &buffer[..size]);
                
                match verify {
                    //authenticated message success...
                    Ok(_message) => {
                        //TODO: message processing
                    },
                    Err(e) => println!("Unable to authenticate and verify message: {}", e)
                }
            }

            //else empty message, do nothing and just wait to fill the receive buffer...
            to_send = Some(socket.recv_from(&mut buffer).await?);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:3156".to_string());

    let socket = UdpSocket::bind(&addr).await?;
    
    let secret_key = aead::SecretKey::default();

    let sbhandler = SquelchBreakHandler {
        socket,
        secretkey: secret_key,
        buffer: vec![0; 1024],
        to_send: None,
    };

    //start the event loop
    sbhandler.run().await?;
    
    Ok(())
}
