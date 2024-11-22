use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:34254");

        // recieves a single datafram message on socket.
        // if 'buf' is too small to hold
        // the message, it will be cut off.
        // after creating a UdpSocket by binding it to a socket address, data can be sent to and recieved from any other socket address
        let mut buf = [0; 10];
        let (amt, src) = socket.recv_from(&mut buf)?;

        // redeclare 'buf' as slice of the recieved data and reverse data back to origin.
        let buf = &mut buf[..amt];
        buf.reverse();
        socket.send_to(buf, &src)?;
        println!("End of execution")
    }
    // close the socket
    Ok(())
}
