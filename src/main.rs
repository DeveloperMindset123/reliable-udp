//mod client;
use tokio::net::UdpSocket;
use std::io;

/// https://docs.rs/tokio/latest/tokio/net/struct.UdpSocket.html
/// https://github.com/MemoriesOfTime/raknet-rs/blob/main/examples/tracing.rs --> see this file for example uses, some of the functions has already been defined.
async fn test() -> io::Result<()> {
   let socket = UdpSocket::bind("0.0.0:8080").await?;
   let mut buf = [0; 1024];
   loop {
    let (len, addr) = socket.recv_from(&mut buf).await?;
    println!("{:?} bytes recieved from {:?}", len, addr);
    let len = socket.send_to(&buf[..len], addr).await?;
    println!("{:?} bytes sent", len);
   }
}

fn main() {
    //test();
}