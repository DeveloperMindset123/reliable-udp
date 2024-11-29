//! Note that the terms "client" and "server" here are purely what we logically associate with them.
//! Technically, they both work the same.
// library to recieve user input
// TODO : add more comments explaining the purpose of each
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
use std::io::stdin;
use std::thread;
use std::time::Instant;

// library that is being used to send reliable UDP packet
// as well as socket binding, error handling
use laminar::{ErrorKind, Packet, Socket, SocketEvent};

// this is where the server is, where message from the client side can be recieved
const SERVER: &str = "127.0.0.1:3000";
//static mut sequenceNumber: u8 = 1;

// Result<(), ErrorKind>
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
     */
    let mut socket = Socket::bind(SERVER)?;
    let (sender, receiver) = (socket.get_packet_sender(), socket.get_event_receiver());

    // define _thread as a closure
    // let _thread = thread::spawn(move || socket.start_polling());

    // a new thread is spawned
    // thread takes in a closure of type FnOnce
    // meaning socket.start_polling() should only execute once
    thread::spawn(move || socket.start_polling());
    let mut seq = 1;

    loop {
        if let Ok(event) = receiver.recv() {
            match event {
                SocketEvent::Packet(packet) => {
                    let msg = packet.payload();

                    if msg == b"Bye!" {
                        break;
                    }

                    // this is where the package lost logic occurs
                    // replaces invalid UTF-8 characters
                    // with valid UTF-8 characters
                    let msg = String::from_utf8_lossy(msg);
                    let ip = packet.addr().ip();

                    println!("Received message {:?} from {:?}", msg, ip);
                    // increment the sequence number by 1
                    //seq = seq + 1;
                    let response = format!("Pong!");
                    sender
                        .send(Packet::reliable_sequenced(
                            packet.addr(),
                            // TODO : this needs to be modified
                            response.as_bytes().to_vec(),
                            Some(seq),
                        ))
                        .expect("This should send");
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
