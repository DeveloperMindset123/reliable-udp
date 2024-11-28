use std::net::UdpSocket;
// this essentially sets up port 4000 to be the listener
// @see https://www.youtube.com/watch?v=sw3IsrKYmzk

/// if we want to send a request from the client side
/// echo -n 'custom string message' 127.0.0.1 4000

fn main() {
    // UDP communication using RUST
    // exposes port 4000
    let socket = UdpSocket::bind("0.0.0.0:4000").unwrap();

    // will print out what port the server is listening to
    println!("Server listening on {}", socket.local_addr().unwrap());

    // specifies the buffer size
    let mut buffer = [0; 1024];
    loop {
        // the open socket recieves the data
        let (size, source) = socket.recv_from(&mut buffer).unwrap();
        // the sent request is converted from bytes to String to be printed out on the terminal
        let request = String::from_utf8_lossy(&buffer[..size]);

        // if the request is sent successfully, will print out the request as well as the soource
        println!("Recieved {} from {}", request, source);

        // determines what the server side message should be
        let response = "Hello from server...!";

        // specifies the socket to send the response to the client side
        // converts the message into bytes and sends it back to the source, which is the client
        socket.send_to(response.as_bytes(), source).unwrap();
    }
}
