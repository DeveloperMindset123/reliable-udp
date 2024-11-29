/// Iteration 1 : commented it out since it was learning purpose
/// 
// use laminar::{DeliveryGuarantee, Packet, Socket};
// use std::any::type_name;
// use std::net::SocketAddr;
// use std::vec::Vec;

// // borrow and initialize
// const SERVER_ADDR: &str = "127.0.0.1:4000";
// const CLIENT_ADDR: &str = "127.0.0.1:3000";

// fn socket_address(address_val: &str) -> SocketAddr {
//     return address_val.parse().unwrap();
// }

// // define the method as a closure
// fn construct_new_packet(rawData: &str, method: impl FnOnce(some_destination : SocketAddr, byte_data : Vec<u8>) -> Packet) -> Packet {
//     let destination : SocketAddr = socket_address(SERVER_ADDR);
//     let payload = rawData.as_bytes();  // datatype : &[u8]
//     //let mut payload_owned = payload.to_owned();

//     println!("{}", type_of(payload));
//     // NOTE : the idea here is the following
//     // since method is a closure
//     // we will pass in the path as the following
//     // Packet::unreliable(destination, payload.to_owned())
//     let packet: Packet = method(socket_address(SERVER_ADDR), payload.to_owned());

//     return packet;
// }

// // custom function to define the type
// fn type_of<T>(_: T) -> &'static str {
//     type_name::<T>()
// }

// // specify the data structure
// // this data structure should not have any values
// // not yet defined
// struct custom_packet {
//     destination: SocketAddr,

// }

// // define the different methods that are available
// // impl custom_packet {}
// fn main() {
//     let mut my_packet = construct_new_packet("Some Raw Data", Packet::unreliable);
//     println!("Packet is : {}", my_packet);
// }
