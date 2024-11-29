/// NOTE : this file isn't being used 
// use bytes::Bytes;
// use futures::{SinkExt, StreamExt};
// use raknet_rs::server::{self, MakeIncoming};

// let socket = tokio::net::UdpSocket::bind("127.0.0.1.0").await?;
// let config = server::Config::new()
//     .send_buf_cap(1024)
//     .server_guid(114514)
//     .advertisement(&"This is the server"[...])
//     ...
// let mut incoming = socket.make_incoming(config);
// let (reader, _) = incoming.next().await.unwrap();
// tokio::pin!(reader);
// let data : Bytes = reader.next().await.unwrap();

// fn some_funcion() {
//     println!("This is another function");
// } 
