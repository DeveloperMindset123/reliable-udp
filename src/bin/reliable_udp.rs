#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_mut)]
use laminar::{DeliveryGuarantee, Packet, Socket, SocketEvent};
use std::any::type_name;
use std::any::Any;
use std::net::SocketAddr;
use std::vec::Vec;
//#![allow(unused_mut)]

const SERVER_ADDR: &str = "127.0.0.1:4000";
const CLIENT_ADDR: &str = "127.0.0.1:123456";

fn construct(method: impl FnOnce() -> Packet) -> Packet {
    method()
}

fn socket_address(address_val: &str) -> SocketAddr {
    return address_val.parse().unwrap();
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
fn send_data() {
    // bind the client socket
    // when SocketAddr is returned by invoking socket_address("IP:PORT")
    // we need to use .unwrap() to remove the container and get the content of the container
    // thus we use .unwrap()
    let mut socket = Socket::bind(socket_address(CLIENT_ADDR)).unwrap();
    let mut packet_object = customPacket::new(socket_address(SERVER_ADDR), "ping".as_bytes(), 1);
    let mut reliable_sequenced = packet_object.construct_reliable_sequenced_udp();

    // send the packet data
    socket.send(reliable_sequenced);
}

/// define the logic for how data should be recieved
fn recieve_data() {
    // setup an UDP socket and bind it the server address
    // similar to how we binded to the client address
    let mut socket = Socket::bind(socket_address(SERVER_ADDR)).unwrap();

    // we recieve the packet now
    // we need to recieve the packet in small chunks
    loop {
        // recieve from the socket and wrap it around a container using Some
        if let Some(result) = socket.recv() {
            match result {
                SocketEvent::Packet(packet) => {
                    let endpint: SocketAddr = packet.addr();
                    let recieved_data: &[u8] = packet.payload();

                    // deserialize the bytes
                    // print out the information
                    // this is basically the "reconstruction phase"
                    // where the bytes are taken and reconstructed to render on the server's end
                    // here we are printing out the endpoit and the length of data we are recieving
                    println!(
                        "Recieved packet from {:?} with length {:?}",
                        endpint,
                        recieved_data.len()
                    );
                }
                _ => {}
            }
            break;
        }
    }
}

fn main() {
    send_data();
}
