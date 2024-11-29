//! Note that the terms "client" and "server" here are purely what we logically associate with them.
//! Technically, they both work the same.
// library to recieve user input
// TODO : add more comments explaining the purpose of each
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
/// #![allow{macros}] is used to suppress distracting warning during compilation
/// comment them out and compile to see the appropriate warning messages if preferred
/// functionality will not be impacted
///
///
/// use libraryRoot::libraryMethod::*
/// the import statements are dependent on how the methods has been implemented
/// std is the standard library within rust itself, doesn't not require downloading any external libraries
/// cargo works similar to npm, it is a package manager
/// but we also use cargo to compile/run our code
///
/// To compile : cargo build --bin final
/// To run (and compile) : cargo run --bin final
/// use std::io::stdin is used to recieve user input
use std::io::stdin;

/// utilizes OS threads to boost performance in exeuction of program
use std::thread;

/// used to keep track of time elapsed
/// used to calculate RTT
use std::time::Instant;

/// library that is being used to send reliable UDP packet
/// as well as socket binding, error handling
/// this library handles the underlying structure of recovering lost packet and ensuring packets are transmitted successfully as intended
use laminar::{ErrorKind, Packet, Socket, SocketEvent};

/// this is where the server is, where message from the client side can be recieved
/// "const" and "static" is used to make variables global
/// 127.0.0.1 is an special IP address to refer to the device itself
/// similar to the self parameter when defining object methods within object oriented programming
const SERVER: &str = "127.0.0.1:3000";

// Result<(), ErrorKind> represents the return type of the function
// this is the server that recieves messages
// the socket this server connects to is 3000, for simplicity

fn server() -> Result<(), ErrorKind> {
    // bind the socket
    /**
     * In Rust, "binding a socket" means associating specific network address
     * like an IP address and port with a newly created socket
     * it essentially tells the operating system that this socket should be reachable at that particular address
     *
     * allowing other applications to connect to it o the network
     * -> Result<(), ErrorKind> allows us to use ? instead of .unwrap()
     * unwrap() is older form of error handling, ? is a newer form of handling errors
     *
     * socket.get_packet_sender() --> Returns a handle to the packet sender which provides a thread-safe way to enqueue packets to be processed. This could be used when the socket is busy running it's polling loop in a seperate thread
     *
     * socket.get_event_reciever() --> returns a handle to the event reciever
     */
    let mut socket = Socket::bind(SERVER)?;
    let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());

    // define _thread as a closure
    // a new thread is spawned
    // thread takes in a closure of type FnOnce
    // meaning socket.start_polling() should only execute once
    // this is automatically executed when the program runs
    thread::spawn(move || socket.start_polling());

    // seq keeps track of the current sequence value of the packet
    let mut seq = 1;

    // the loop is used to recieve the packets
    // the network may not be capable of sending all the packets at once
    // this depends on how much data it can send within a small timeframe
    // therefore, we need to use a loop based statement
    loop {
        // equivalent to using try/catch in javascript
        // logic defining what the server should do upon successfully recieving the packets
        if let Ok(event) = receiver.recv() {
            match event {
                SocketEvent::Packet(packet) => {
                    let msg = packet.payload();

                    // no response will be sent if user inputs Bye!
                    if msg == b"Bye!" {
                        break;
                    }

                    // this is where the package lost logic occurs
                    // replaces invalid UTF-8 characters
                    // with valid UTF-8 characters
                    let msg = String::from_utf8_lossy(msg);

                    // extracts the ip
                    let ip = packet.addr().ip();

                    println!("Received message {:?} from {:?}", msg, ip);
                    // increment the sequence number by 1
                    //seq = seq + 1;
                    let response = format!("Pong!");
                    sender
                        .send(Packet::reliable_sequenced(
                            // sends packet in reliable + orderly manner
                            packet.addr(),
                            // TODO : this needs to be modified
                            response.as_bytes().to_vec(),
                            Some(seq),
                        ))
                        .expect("This should send"); // error handler in the event packets aren't send
                }
                SocketEvent::Timeout(address) => {
                    println!("Client timed out: {}", address);
                }
                _ => {}
            }
        }
    }

    Ok(())
}

/// there's 41 different variants when it comes to handling errors
/// essentially helps us identify what kind of error is occuring
fn client(number_of_requests: u8) -> Result<(), ErrorKind> {
    // this is the client side socket/ip
    // @127.0.0.1 : is a special ip address known as "the loopback address"
    // it is used by the computer to refer to itself
    // @:* represents the port number, this can be any available port within the operating system
    let addr = "127.0.0.1:12352";
    let mut socket = Socket::bind(addr)?;
    println!("Connected on {}", addr);
    let mut seq = 1;

    // when we bind a socket
    // the address gets "wrapped" around, essentially imagine wrapping a physical object with a gift wrapper or placing a content withing a container
    // therefore, we need to "unwrap" it
    // thus the use of unwrap()
    // parse() helps convert
    let server = SERVER.parse().unwrap();

    println!("Type a message and press Enter to send. Send `Bye!` to quit.");

    let stdin = stdin();
    let mut s_buffer = String::new();
    loop {
        s_buffer.clear();
        stdin.read_line(&mut s_buffer)?;
        let mut line = s_buffer.replace(|x| x == '\n' || x == '\r', "");
        // send 10 ping messages back to back
        // it's not limited to just ping messages
        // after sending the specified number of requests here back to back
        // the server will be sending waiting for acknowledgement for
        for i in 1..number_of_requests + 1 {
            let now = Instant::now();
            let string = i.to_string();
            //line.push_str(&string);

            // send reliable sequence data
            socket.send(Packet::reliable_sequenced(
                server,
                line.clone().into_bytes(),
                Some(i),
            ))?;

            socket.manual_poll(Instant::now());

            if line == "Bye!" {
                break;
            }

            seq = seq + 1;
            match socket.recv() {
                Some(SocketEvent::Packet(packet)) => {
                    if packet.addr() == server {
                        // prints out the message recieved on the server side
                        // handles packet loss and reconstructs packets as needed
                        // unpack what the server sent
                        // server should respond with Ping
                        println!(
                            "{}, {} RTT {:?}",
                            String::from_utf8_lossy(packet.payload()),
                            i,
                            now.elapsed()
                        );
                    } else {
                        // if sender cannot be verified
                        // print out unknwon sender
                        println!("Unknown sender.");
                    }
                }
                Some(SocketEvent::Timeout(_)) => {}
                _ => println!("Pong! {:?}, RTT : {:?}", i, now.elapsed()),
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), ErrorKind> {
    // used to take in user input
    let stdin = stdin();

    println!("Please type in `server` or `client`.");

    let mut s = String::new();
    stdin.read_line(&mut s)?;

    if s.starts_with('s') {
        println!("Starting server..");
        server()
    } else {
        println!("Starting client..");
        client(30)
    }
}
