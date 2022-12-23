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

// Message Types
static KEY_EXCHANGE_REQ: u8 = 1;

struct SquelchBreakHandler {
    uuid: Uuid,
    socket: UdpSocket,
    secretkey: aead::SecretKey,
    buffer: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
    message_queue: VecDeque<Message>,
    peers: Vec<Peer>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash)]
struct Peer {
    uuid: String,
    ip: String,
}

//TODO: Routing impl 
/*
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash)]
struct RouteTable {
    //TODO: Link-State like implementation???

}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash)]
struct Route {
    peer: Peer
    distance: u8,
}*/

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash)]
struct Message {
    message_type: u8,
    uuid: String, //sender uuid
    message_payload: String, //message payload
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
            mut peers,
        } = self;

        loop {
            if let Some((size, _sender)) = to_send {
                info!("Received message from: {:#?}", _sender);

                let msg = String::from_utf8(Vec::from(&buffer[..size]));
                //println!("{:#?}", msg);

                match msg {
                    Ok(message) => {
                        println!("{:#?}", message);
                        if message.contains("\"message_type\": 1") {
                            process_key_exchange_request(&message);
                        }

                    },
                    _ => error!("Could not parse message"),
                }
                
            }

            //else empty message, do nothing and just wait to fill the receive buffer...
            to_send = Some(socket.recv_from(&mut buffer).await?);
        }
    }
}

//process key exchange request
fn process_key_exchange_request(raw_message: &str)  {
    let kex_msg: Result<Message, serde_json::Error>  = serde_json::from_str(raw_message);
    match kex_msg {
        Ok(msg) => println!("{:#?}", msg),
        _ => error!("Error deserializing message")
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
    
    let mut message_queue: VecDeque<Message> = VecDeque::new();
    let mut peers: Vec<Peer> = Vec::new();

    let sbhandler = SquelchBreakHandler {
        uuid,
        socket,
        secretkey,
        buffer: vec![0; 1024], //1024 bytes buffer
        to_send: None,
        message_queue: message_queue,
        peers,
    };

    //start the event loop
    sbhandler.run().await?;
    
    Ok(())
}
