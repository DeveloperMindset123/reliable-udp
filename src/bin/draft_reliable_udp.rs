#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_must_use)]
#![allow(non_snake_case)]
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

/// semi working code, duplicate packets are transmitted
/// not all acknowlegement messages are being returned at oance

const SERVER_ADDR: &str = "127.0.0.1:4000";
const CLIENT_ADDR: &str = "127.0.0.1:3000";

// this function isn't being used
fn construct(method: impl FnOnce() -> Packet) -> Packet {
    method()
}

// NOTE : this function might be causing errors
// most likely due to the semicolon, requires further testing
// logically seems sound but compiler panics
fn socket_address(address_val: &str) -> SocketAddr {
    return address_val.parse().unwrap();
}

/// alternative to socket_address
/// reference that I was following defined it like this
/// socket_address and the two getter functions below have the same functionality
fn get_server_address() -> SocketAddr {
    SERVER_ADDR.parse().unwrap()
}

fn get_client_address() -> SocketAddr {
    CLIENT_ADDR.parse().unwrap()
}

/// part of learning how structs/implementations work within Rust
struct customPacket<'a> {
    destination: SocketAddr, // need to call on socket_addr() with the socket value passed in
    payload: &'a [u8],       // need to pass in "ping".as_bytes()
    sequence_number: u8,
}

/// method has the following methods
/// 1. unreliable_sequenced
/// 2. reliable_sequenced : this is what we need to implement
/// 3. reliable_ordered
/// 4. unreliable
/// TODO : these methods are semi-working, need to determine appropriate fix for why they aren't working as intended.
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

    client.send(Packet::unreliable(
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
        //Some(i),
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
                println!("Pong!");
            }
            _ => {}
        }
    }

    // define a function handle sending and recieving packets
    // the only value this function should be recieving is the current sequence number by which the packet should be sent

    // TODO : Figure out why this function isn't working
    // fn reliable_udp() {
    //     let mut server = Socket::bind(get_client_address()).unwrap();

    //     let mut client = Socket::bind(get_server_address()).unwrap();

    //     // start the socket for the client side
    //     // thread::spawn(client.start_polling());
    //     client.manual_poll(Instant::now());

    //     // start the thread for the server side
    //     // thread::spawn(server.start_polling());
    //     server.manual_poll(Instant::now());

    //     // create the reliable_sequenced packets
    //     let data = "Ping".as_bytes();
    //     for sequenceNumber in 1..11 {
    //         // get the packet_sender for client
    //         let packet_sender = client.get_packet_sender();

    //         // get the event reciever method from socket
    //         let event_reciever = server.get_event_receiver();
    //         println!("Current iteration {:?}", sequenceNumber);

    //         let reliable_sequenced = Packet::reliable_sequenced(
    //             get_server_address(),
    //             data.to_owned(),
    //             Some(sequenceNumber.to_owned()),
    //         );

    //         packet_sender.send(reliable_sequenced).unwrap();
    //         // _thread();
    //         // _thread_server();

    //         // logic for reciveing packets
    //         // wait until a socket event occurs
    //         let result = event_reciever.recv();

    //         match result {
    //             // this is similar to try/catch block in typescript
    //             Ok(socket_event) => match socket_event {
    //                 SocketEvent::Packet(packet) => {
    //                     let endpoint: SocketAddr = packet.addr();
    //                     let recieved_data: &[u8] = packet.payload();
    //                 }
    //                 SocketEvent::Connect(connect_event) => {
    //                     println!("Pong! {:?}", Some(sequenceNumber));
    //                 }
    //                 SocketEvent::Timeout(timeout_event) => {
    //                     println!("Timeout!");
    //                 }
    //                 SocketEvent::Disconnect(disconnect_event) => {
    //                     println!("Disconnected client");
    //                 }
    //             },
    //             Err(e) => {
    //                 println!("Something went wrong when recieving, error : {:?}", e);
    //             }
    //         }
    //         println!("Server has recieved data successfully");

    //         reliable_udp();
    //     }
    // }
}

// TODO : look into this --> https://github.com/TimonPost/laminar/blob/master/examples/simple_udp.rs

#[derive(Debug, Serialize, Deserialize)]
enum PacketType {
    PacketContent { payload: String },
    PacketHeader { sequence_number: u32 },
}
