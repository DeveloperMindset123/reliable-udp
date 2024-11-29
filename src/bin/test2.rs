/// iteration 3, not fully functional 
#![allow(unused_doc_comments)] // allow doc comments
#![allow(unused_imports)] // disable unused import warning
#![allow(unused_variables)] // disable unused variable warning
#![allow(unused_mut)] // disable unused mutable variable warning
use mio::net::UdpSocket;
use rudp::Endpoint;
use std::any::type_name;
use std::net::SocketAddr;
use std::result::Result;

// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
/// @see https://www.youtube.com/watch?v=5LdnfzFdWhE --> understanding difference between packages, crates, mod for internal, use for external,
/// @see https://www.youtube.com/watch?v=gi0AQ78diSA&t=224s --> to better understand how struct works
///
// @see https://docs.rs/mio/latest/mio/net/struct.UdpSocket.html --> this documentation explains how to construct a peer address for rust

/// understanding the purpose of dyn --> dyn is short for "dynamic" and refers to the fact that the trait objects perform dynamic dispatch.
///
/// Result<()> does not represent a void function in Rust
///
/// while () by itself can be considered a "unit" type representing the absence of a value (similar to void return)
/// Result<()> actually indicates a function that can either return a successful result with no value (Ok(())) or an error (Err(E)) where E represents the error type
///
/// @see https://users.rust-lang.org/t/best-practices-for-unwrap/101335
///
/// "traits" work similar to interfaces in an abstract way
/// allowing us to define the types for structs (think of structs as something that's built on top of traits, where custom types may be neccessary)
/// think traits as means of initializing an interface
/// struct works similar to actual typescript interfaces
/// impl helps actually define the logic behind the implementation
/// @see https://www.youtube.com/watch?v=6fwDwJodJrg&t=56s --> to better understand the difference between Traits vs Trait Bounds and Lifetime annotations
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // if you want to know if it was successful you can either use ? to propagate the error to the caller (if your function didn't return, the call succeeded), or use a match to explicitly handle both the success and error cases --> from discord server.

    // TODO : see if you can implement them using ? instead of .unwrap(), as means of propagating errors
    // let sender_socket_address = convert_to_socket_addr("127.0.0.1:4000");
    //let echoer_socket_address = convert_to_socket_addr("127.0.0.1:3000");
    //let mut sender_socket = UdpSocket::bind(&sender_socket_address).unwrap();
    //let mut echoer_socket = UdpSocket::bind(&echoer_socket_address);
    let mut sender_socket = UdpSocket::bind(&"127.0.0.1:4000".parse()?)?;

    /// Not fully sure why this binding approach doesn't work
    /// let mut echoer_socket = UdpSocket::bind(&"127.0.0.1:3000".parse()?)?; --> unused port
    ///
    /// instead, the port has been set manually for the client side
    let echoer_socket_address = convert_to_socket_addr("127.0.0.1:3000");
    sender_socket.connect(echoer_socket_address).unwrap();
    /// causing error beyond this point regarding traits.
    ///let mut endpoint = Endpoint::new("127.0.0.1:4000");
    // Our socket was created, but we should not use it before checking it's readiness.
    // ensure that the format has {:?} as part of the string formatting
    // understanding the purpose of {} vs {:?}
    // unlike the standard {} format specifier, which might only print a basic representation of a value, {:?} will display the full type infomration along with the data
    // making it easier to understand complex data structures --> meaning for more complex style of data, use {:?} instead.

    // socket_type simply stores the type of the function
    let socket_type = type_of(sender_socket); // returns : mio::net::udp::UdpSocket
                                              // specifies the server to connect to?
                                              // NOTE : it's better convention to use ? instead of .unwrap()

    // if we do not use connect here, Sender and Echoer would need to call send_to and recv_from, respectively
    //let mut endpoint = rudp::Endpoint::new(sock);

    // we need a poll to check if SENDER is ready to be written into, and if ECHOER is ready to be read from
    Ok(())
}

/// pass by reference
/// when you pass parameters by reference
/// unlike value parameters, a new storage location is not created for these parameters
/// the reference parameters represent the same memory location as the actual parameters that are supplied to the method.
/// parameter values can be passed by reference by prefixing the variable name with an &
/// parameters can be immutable, since they generally remain unchanged once passed in once.
/// @see https://stackoverflow.com/questions/28255861/convert-string-to-socketaddr --> function explaining how the conversion logic works
fn convert_to_socket_addr(server_string: &str) -> std::net::SocketAddr {
    let server_details = server_string;
    let server_parsed: SocketAddr = server_details
        .parse()
        .expect("Unable to parse socket address");
    //println!("{:?}", server_parsed);
    return server_parsed;
}

/// @see https://users.rust-lang.org/t/how-check-type-of-variable/33845
/// T serves as a generic placeholder for any type
/// alternative : type_name_of_val exists and does the same thing
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
