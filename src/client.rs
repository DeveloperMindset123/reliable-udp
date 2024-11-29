/// files not being used
/// 
/// saved for future reference 
// use bytes::Bytes;
// use futures::{SinkExt, StreamExt};
// use raknet_rs::client::{self,ConnectTo};
// use raknet_rs::Reliabillity;

// let socket = tokio::net::UdpSocket::bind("0.0.0.0:0").await?;
// let config = client::Config::new()
//     .send_buf_cap(1024)
//     .client_guid(1919810)
//     ...
// let (_, writer) = socket.connect_to(<addr>, config).await?;
// tokio::pin!(writer);
// writer.send(Message::new(Reliabillity::Reliable, 0, Bytes::from_static(b"Hello, who's there?")))
// .await?;