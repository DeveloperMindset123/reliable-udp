#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
use laminar::{DeliveryGuarantee, Packet, Socket};
use std::any::type_name;
use std::any::Any;
use std::net::SocketAddr;
use std::vec::Vec;
//#![allow(unused_mut)]

const SERVER_ADDR: &str = "127.0.0.1:4000";

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

fn main() {
    // since no constructors are needed
    // since the interface doesn't have any such types
    // let payload_data = "some data".as_bytes();
    // let my_packet = customPacket::get_unreliable_udp(
    //     socket_address(SERVER_ADDR),
    //     payload_data.to_owned(),
    //     Packet::unreliable,
    // );

    let mut packet_object = customPacket::new(socket_address(SERVER_ADDR), "ping".as_bytes(), 1);
    let mut reliable_sequenced = packet_object.construct_reliable_sequenced_udp();

    println!("{:?}", reliable_sequenced);
}
