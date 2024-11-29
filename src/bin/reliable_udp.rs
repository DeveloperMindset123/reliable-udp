#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_mut)]
use bincode::{deserialize, serialize};
use laminar::{DeliveryGuarantee, Packet, Socket, SocketEvent};
use serde_derive::{Deserialize, Serialize};
use std::any::type_name;
use std::any::Any;
use std::net::SocketAddr;
use std::thread;
use std::time::Instant;
use std::vec::Vec;
//#![allow(unused_mut)]

const SERVER_ADDR: &str = "127.0.0.1:4000";
const CLIENT_ADDR: &str = "127.0.0.1:3000";

fn construct(method: impl FnOnce() -> Packet) -> Packet {
    method()
}

// NOTE : this function might be causing errors
// most likely due to the semicolon, requires further testing
// logically seems sound but compiler panics
fn socket_address(address_val: &str) -> SocketAddr {
    return address_val.parse().unwrap();
}

// using the default getters instead
// old functions, no longer relevant

// fn server_address() -> SocketAddr {
//     SERVER_ADDR.parse().unwrap()
// }

// fn client_address() -> SocketAddr {
//     CLIENT_ADDR.parse().unwrap()
// }

struct customPacket<'a> {
    destination: SocketAddr, // need to call on socket_addr() with the socket value passed in
    payload: &'a [u8],       // need to pass in "ping".as_bytes()
    sequence_number: u8,
}

/// method has the following methods
/// 1. unreliable_sequenced
/// 2. reliable_sequenced
/// 3. reliable_ordered
/// 4. unreliable
impl customPacket<'_> {
    // define the constructor
    // constructor is parametrized
    fn new(destination: SocketAddr, payload: &[u8], seq: u8) -> customPacket {
        customPacket {
            destination: destination,
            payload: payload,
            sequence_number: seq,
        }
    }
    // we will not be defining any kind of constructors here
    // these would be class_instance based properties in this case
    // define them as methods to improve readabillity
    fn construct_unreliable_udp_packet(
        &mut self,
        // destination_addr: SocketAddr,
        // payload_data: Vec<u8>,
        // packetPath: Box<dyn Any>,
    ) -> Packet {
        // @see https://forum.dfinity.org/t/passing-async-function-or-closure-as-parameter/21785/3
        // @see https://rustyyato.github.io/rust/syntactic/sugar/2019/01/17/Closures-Magic-Functions.html

        // using closure is optional
        // I used primarily for learning purpose
        let unreliable_udp = move |destination: SocketAddr, payload: Vec<u8>| {
            Packet::unreliable(destination, payload)
        };

        // invoke the closure
        // based on the parameters specified during the function call
        return unreliable_udp(self.destination, self.payload.to_owned());
    }

    // the following methods below requires 3 arguments
    fn construct_reliable_sequenced_udp(&mut self) -> Packet {
        // this is method 2
        // we have to make slight adjustment
        // since previously the closure handled it for us
        return Packet::reliable_sequenced(
            self.destination,
            self.payload.to_owned(),
            Some(self.sequence_number),
        );
    }

    fn construct_reliable_ordered_udp(&mut self) -> Packet {
        return Packet::reliable_ordered(
            self.destination,
            self.payload.to_owned(),
            Some(self.sequence_number),
        );
    }

    fn construct_unreliable_sequenced_udp(&mut self) -> Packet {
        return Packet::unreliable_sequenced(
            self.destination,
            self.payload.to_owned(),
            Some(self.sequence_number),
        );
    }
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    // uncomment them after testing if the other function doesn't work
    // let mut server = Socket::bind(server_address()).unwrap();
    // let mut client = Socket::bind(client_address()).unwrap();

    let mut server = Socket::bind(socket_address(SERVER_ADDR)).unwrap();

    let mut client = Socket::bind(socket_address(CLIENT_ADDR)).unwrap();

    for i in 1..11 {
        client.send(Packet::reliable_ordered(
            socket_address(SERVER_ADDR),
            // TODO : wrap this around a sequence number
            // serialize formats the data so that it can be sent over the network in the form of bytes
            // essentially, the raw string data is broken down into bytes
            // 3 things happening here
            // Packet::reliable_sequenced takes in three things
            // param 1 : socket address for the server --> since client is the one sending the data
            // second is the data in bytes, where we take the raw string data and convert it to bytes
            serialize(&PacketType::PacketContent {
                // borrow data
                // send data
                // we also want to print out the value of the current sequence number
                payload: String::from(format!("Ping!")),
            })
            .unwrap(),
            // sequence number based on the current value of the loop
            Some(i),
        ));

        // send the queued data operation
        client.manual_poll(Instant::now());

        // check the server for any new packet
        server.manual_poll(Instant::now());

        // === result ===
        // we need to take the bytes in increments
        // in the event that entire packet can be sent through directly
        while let Some(pkt) = server.recv() {
            match pkt {
                SocketEvent::Packet(pkt) => {
                    // used to print out JSON data format
                    // this will print out the content of the payload in a JSON format
                    println!["{:?}", deserialize::<PacketType>(pkt.payload()).unwrap()];
                    // reply with Pong!
                    println!("Pong! {:?}", i);
                }
                _ => {}
            }
        }
    }
}

// TODO : look into this --> https://github.com/TimonPost/laminar/blob/master/examples/simple_udp.rs

#[derive(Debug, Serialize, Deserialize)]
enum PacketType {
    PacketContent { payload: String },
    PacketHeader { sequence_number: u32 },
}
