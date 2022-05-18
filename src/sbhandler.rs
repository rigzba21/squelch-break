//use std::process;
use std::error::Error;
use std::net::SocketAddr;
use std::{env, io};
use tokio::net::UdpSocket;
use orion::aead;
use orion::kex::*;
use std::collections::VecDeque;
//use std::collections::hash_map::DefaultHasher;
//use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{Utc};
use log::{debug, error, log_enabled, info, Level};


struct SquelchBreakHandler {
    uuid: Uuid,
    socket: UdpSocket,
    secretkey: aead::SecretKey,
    buffer: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
    message_queue: VecDeque<Message>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash)]
pub struct Message {
    message_id: u64, //hash value of sender_id + message_payload 
    uuid: String, //sender uuid
    message_payload: String, //message payload
    timestamp: i64    
}

impl SquelchBreakHandler {
    //TODO: distribute aead::SecretKey to sbrunner(s)

    //run the event loop
    async fn run(self) -> Result<(), io::Error> {
        let SquelchBreakHandler {
            uuid,
            socket,
            secretkey,
            mut buffer,
            mut to_send,
            mut message_queue,
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

//initialize an Ephemeral Private Key Exchange session 
//for securely transmitting private key to peers
fn init_peers_key_exchange(peers: Vec<SocketAddr>) {
   let ephemeral_session_server = EphemeralServerSession::new().unwrap();

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "0.0.0.0:3156".to_string());

    let uuid = Uuid::new_v4();

    let socket = UdpSocket::bind(&addr).await?;
    info!("sbhandler initialized and listening at {}", addr);

    let secretkey = aead::SecretKey::default();

    let sbhandler = SquelchBreakHandler {
        uuid,
        socket,
        secretkey,
        buffer: vec![0; 1024], //1024 bytes buffer
        to_send: None,
        message_queue: VecDeque::new(),
    };

    //start the event loop
    sbhandler.run().await?;
    
    Ok(())
}
