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
fn socket_address(address_val: &str) -> SocketAddr {
    return address_val.parse().unwrap();
}

// using the default function we have been provided
fn server_address() -> SocketAddr {
    SERVER_ADDR.parse().unwrap()
}

fn client_address() -> SocketAddr {
    CLIENT_ADDR.parse().unwrap()
}

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

/// send data over UDP
// fn send_data() {
//     // bind the client socket
//     // when SocketAddr is returned by invoking socket_address("IP:PORT")
//     // we need to use .unwrap() to remove the container and get the content of the container
//     // thus we use .unwrap()
//     let mut socket = Socket::bind(socket_address(CLIENT_ADDR)).unwrap();
//     let mut packet_object = customPacket::new(socket_address(SERVER_ADDR), "ping".as_bytes(), 1);
//     //let mut reliable_sequenced = packet_object.construct_reliable_sequenced_udp();
//     let packet_sender = socket.get_packet_sender();

//     // start the socket
//     // this will start a pill mechanism to recieve and send message
//     let _thread = thread::spawn(move || socket.start_polling());
//     let random_string = "some_string".as_bytes();
//     let mut reliable_sequenced = Packet::reliable_sequenced(
//         socket_address(SERVER_ADDR),
//         random_string.to_owned(),
//         Some(2),
//     );
//     return packet_sender.send(reliable_sequenced).unwrap();
// }

// /// define the logic for how data should be recieved
// fn recieve_data() {
//     // setup an UDP socket and bind it the server address
//     // similar to how we binded to the client address
//     let mut socket = Socket::bind(socket_address(SERVER_ADDR)).unwrap();

//     let event_receiver = socket.get_event_receiver();
//     // Starts the socket, which will start a poll mechanism to receive and send messages.
//     let _thread = thread::spawn(move || socket.start_polling());

//     // Waits until a socket event occurs
//     let result = event_receiver.recv();

//     match result {
//         Ok(socket_event) => match socket_event {
//             SocketEvent::Packet(packet) => {
//                 let endpoint: SocketAddr = packet.addr();
//                 let received_data: &[u8] = packet.payload();
//             }
//             SocketEvent::Connect(connect_event) => {
//                 println!("client connected successfully");
//             }
//             SocketEvent::Timeout(timeout_event) => {
//                 println!("client timed out");
//             }
//             SocketEvent::Disconnect(disconnect_event) => {
//                 println!("client has been disconnected.");
//             }
//         },
//         Err(e) => {
//             println!("Something went wrong when receiving, error: {:?}", e);
//         }
//     }
// }

fn main() {
    let mut server = Socket::bind(server_address()).unwrap();
    let mut client = Socket::bind(client_address()).unwrap();

    client.send(Packet::unreliable(
        socket_address(SERVER_ADDR),
        serialize(&PacketType::PacketContent {
            // borrow data
            payload: String::from("Ping!"),
        })
        .unwrap(),
    ));

    // send the queued data operation
    client.manual_poll(Instant::now());

    // check the server for any new packet
    server.manual_poll(Instant::now());
}

// TODO : look into this --> https://github.com/TimonPost/laminar/blob/master/examples/simple_udp.rs

#[derive(Debug, Serialize, Deserialize)]
enum PacketType {
    PacketContent { payload: String },
    PacketHeader { sequenceNumber: u32 },
}
