#![allow(unused_imports)] // this will stop thorwing uneccessary warning for imports that aren't used
use laminar::{DeliveryGuarantee, Packet, Socket};
// use std::any::type_name::T;
use options::Options;
use rhai::{Dynamic, Engine};
use std::borrow::ToOwned;
use std::error::Error;
use std::net::SocketAddr; // import the SocketAddr type
use std::ops::FnOnce;
use std::thread;

/// @see https://docs.rs/laminar/latest/laminar/ --> Laminar Docs
/// functions that you can pass in --> for
///
/// the "?" operator is used to concisely handle potential errors
/// within a function by early returning if an Option or Result value contains an Err or None variant (In this case, the function's return type is Result<(), Box<dyn Error>>)
/// effectively making it preferred method over unwrap()
/// because it provides a more explicit and readable way to handle errors without causing the program to panic/crash
/// in the event that an error were to occur
///
/// String wraps and manages a dynamically allocated str as a backing storage
/// since str cannot be resized, String will dynamically allocate/deallocate memory
/// meaning str is used for string of fixed size
/// String is used for string of variable size
///
/// understanding the difference between str and &str
/// A &str is thus a reference directly into the backing storage of the String
/// while &String is a reference to the "wrapper" object
/// &str can be used for Subtrings
/// &String references always the whole string
/// the socket address of where the server is located
/// the "const" keyword is used to define global variables
const SERVER_ADDR: &str = "127.0.0.1:12345";

// the client address from where the data is sent
const CLIENT_ADDR: &str = "127.0.0.1:123456";

// define a function to convert from &str to SocketAddr
// the parameter it accepts is a pass by reference value
fn socket_address(address_val: &str) -> SocketAddr {
    return address_val.parse().unwrap();
}

/// define a function that will help construct the packet
/// this function will take the raw string data
/// and convert it into bytes
/// the return type is a Packet
/// method determines what kind of data that should be sent out
/// there are several options for method, as listed below
/// method can be one of the following 5 listed below
/// 1. unreliable
/// 2. unreliable_sequenced
/// 3. reliable_sequenced
/// 4. reliable_ordered
/// 5. reliable_unordered
pub fn construct_new_packet(rawData: &str, method: FnOnce() -> Packet) {
    // initialize the destination address of the packet
    let destination: SocketAddr = socket_address(SERVER_ADDR);

    // construct the payload for the packet
    // will throw an error if we attempt to pass
    // in something other than a string
    let payload = rawData.as_bytes();

    // construct the packet using the payload
    // method will be based
    let packet: Packet = Packet::unreliable(destination, payload.to_owned());

    packet // same as writting return packet, the "return" statement does not need to be explicitly mentioned in rust
}

fn main() -> Result<(), Box<dyn Error>> {
    // bind the socket
    // this is the server address
    let mut server_socket = Socket::bind(socket_address(SERVER_ADDR))?;
    let mut client_socket = Socket::bind(socket_address(CLIENT_ADDR))?;
    let packet_sender = server_socket.get_packet_sender();

    // start the socket
    // which will start a poll mechanism to recieve and send messages
    // @see https://doc.rust-lang.org/std/thread/fn.spawn.html#:~:text=Function%20std%3A%3Athread%3A%3Aspawn&text=Spawns%20a%20new%20thread%2C%20returning,to%20join%20the%20spawned%20thread.

    // above link explains what the purpose of thread::spawn is
    // Spawns create a new thread, returning JoinHandle for it
    // The join handle provides a join method that can be used to join the spawned thread
    // if threads aren't joined or detached, this will lead to resource leak
    let _thread = thread::spawn(move || server_socket.start_polling());

    // define the bytes to send
    // TODO : replace with 10 ping messages
    let bytes = vec!["Apple".as_bytes(), "Oranges".as_bytes()];

    // Creates packets with different reliabillities
    // our destination in this case is the server_socket
    // we will be sending the packet from the client side

    // TODO : FIX THIS
    //let unreliable = Packet::unreliable(socket_address(SERVER_ADDR), bytes);
    //let reliable_unordered = Packet::reliable_unordered(server_socket, bytes);

    // species on which stream and how to order our packets
    // species
    //let packet1 = construct_new_packet("ping", unreliable);

    // ensure Ok(()) is present since that is required
    // for function with return type
    // Result<(), Box<dyn Error>>
    Ok(())
}

/// consider the following function that I was provided on discord
///
/// construct acts as a wrapper
/// TODO : Understand this completely
/// @see https://doc.rust-lang.org/std/ops/trait.FnOnce.html
fn construct(method: impl FnOnce() -> Packet) -> Packet {
    method()
}

/// struct is used to define types or declare pre-existing types
struct Packet_custom;

/// impl works similar to class
/// @see https://stackoverflow.com/questions/71816682/method-return-type-vs-self --> to understand the difference between Self and ()
impl Packet_custom {
    fn with_stream_id(stream_id: Option<u8>) -> Self {
        Packet_custom
    }
    fn without_stream_id() -> Self {
        Packet_custom
    }
}

/// originally named main(), changed to pseudo_main()
/// since the function has already been previously defined.
/// this function is mainly for reference only
fn pseudo_main() {
    construct(|| Packet::without_stream_id());
    construct(|| Packet::with_stream_id(None));
}
