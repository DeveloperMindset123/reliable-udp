// // we can directly import tokio::main using use tokio::main to use for testing purposes

// /// @see https://doc.rust-lang.org/std/net/struct.UdpSocket.html --> see how this can be simplified.
// #![allow(clippy::print_stdout)]
// use std::collections::HashMap;
// use std::error::Error;
// use std::net::SocketAddr;
// use std::process::exit;
// use std::time::Duration;
// use bytes::Bytes;
// use fastrace::collector::{SpanContext, SpanId, SpanRecord, TraceId};
// use fastrace::Span;
// use futures::{SinkExt, StreamExt};
// use raknet_rs::client::{self, ConnectTo};
// use raknet_rs::opts::TraceInfo;
// use raknet_rs::server::{self, MakeIncoming};
// use raknet_rs::{Message, Reliability};
// use tokio::net::UdpSocket;   // only tokio based import being made explicitly

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let (reporter, spans) = fastrace::collector::TestReporter::new();
//     fastrace::set_reporter(
//         reporter,
//         fastrace::collector::Config::default().report_before_root_finish(true),
//     );

//     let socket = UdpSocket::bind("127.0.0.1:0").await?;
//     let local_addr = socket.local_addr()?;
//     let mut incoming = socket.make_incoming(
//         server::Config::new()
//             .send_buf_cap(1024)
//             .sever_guid(114514)
//             // this is causing error
//             .advertisement("Hello, I am proxy server")
//             .min_mtu(500)
//             .max_mtu(1400)
//             .support_version(vec![9, 11, 13])
//             .max_pending(64),
//     );

//     tokio::spawn(async move {
//         loop {
//             let (reader, writer) = incoming.next().await.unwrap();
//             tokio::spawn(async move {
//                 tokio::pin!(reader);
//                 tokio::pin!(writer);
//                 loop {
//                     if let Some(data) = reader.next().await {
//                         let trace_id = reader.last_trace_id().unwrap_or_else(|| {
//                             println!("Please run with `--features fastrace/enable` and try again");
//                             exit(0)
//                         });
//                         let root_span = Span::root(
//                             "user root span",
//                             SpanContext::new(trace_id, SpanId::default()),
//                         );
//                         // do something with data
//                         tokio::time::sleep(Duration::from_millis(10)).await;
//                         let _span = Span::enter_with_parent("user child span", &root_span);
//                         writer
//                             .send(Message::new(Reliability::ReliableOrdered, 0, data))
//                             .await
//                             .unwrap();
//                         continue;
//                     }
//                     break;
//                 }
//             });
//         }
//     });

//     client(local_addr).await?;

//     fastrace::flush();
//     display(spans.lock().clone());
//     Ok(())
// }

// async fn client(addr: SocketAddr) -> Result<(), Box<dyn Error>> {
//     let socket = UdpSocket::bind("0.0.0.0:0").await?;
//     let (src, dst) = socket
//         .connect_to(
//             addr,
//             client::Config::new()
//                 .send_buf_cap(1024)
//                 .mtu(1000)
//                 .client_guid(1919810)
//                 .protocol_version(11),
//         )
//         .await?;
//     tokio::pin!(src);
//     tokio::pin!(dst);
//     dst.send(Message::new(
//         Reliability::ReliableOrdered,
//         0,
//         Bytes::from_static(b"User pack1"),
//     ))
//     .await?;
//     dst.send(Message::new(
//         Reliability::ReliableOrdered,
//         0,
//         Bytes::from_static(b"User pack2"),
//     ))
//     .await?;
//     let pack1 = src.next().await.unwrap();
//     let pack2 = src.next().await.unwrap();
//     assert_eq!(pack1, Bytes::from_static(b"User pack1"));
//     assert_eq!(pack2, Bytes::from_static(b"User pack2"));
//     Ok(())
// }

// fn display(spans: Vec<SpanRecord>) {
//     let spans_map: HashMap<SpanId, SpanRecord> = spans
//         .iter()
//         .map(|span| (span.span_id, span.clone()))
//         .collect();
//     let adjacency_lists: HashMap<TraceId, HashMap<SpanId, Vec<SpanId>>> = spans.iter().fold(
//         std::collections::HashMap::new(),
//         |mut map,
//          SpanRecord {
//              trace_id,
//              span_id,
//              parent_id,
//              ..
//          }| {
//             map.entry(*trace_id)
//                 .or_default()
//                 .entry(*parent_id)
//                 .or_default()
//                 .push(*span_id);
//             map
//         },
//     );
//     fn dfs(
//         adjacency_list: &HashMap<SpanId, Vec<SpanId>>,
//         spans: &HashMap<SpanId, SpanRecord>,
//         span_id: SpanId,
//         depth: usize,
//         last: bool,
//     ) {
//         let span = &spans[&span_id];
//         let mut properties = String::new();
//         for (key, value) in &span.properties {
//             properties.push_str(&format!("{}: {}, ", key, value));
//         }
//         let mut events = String::new();
//         for ev in &span.events {
//             events.push_str(&format!("'{}'", ev.name));
//         }
//         let prefix = if depth == 0 {
//             String::new()
//         } else if last {
//             "╰".to_owned() + &"─".repeat(depth) + " "
//         } else {
//             "├".to_owned() + &"─".repeat(depth) + " "
//         };
//         println!(
//             "{}{}({}{{{}}}) [{}us]",
//             prefix,
//             span.name,
//             properties,
//             events,
//             span.duration_ns as f64 / 1_000.0,
//         );
//         if let Some(children) = adjacency_list.get(&span_id) {
//             for (i, child) in children.iter().enumerate() {
//                 dfs(
//                     adjacency_list,
//                     spans,
//                     *child,
//                     depth + 1,
//                     i == children.len() - 1 && last,
//                 );
//             }
//         }
//     }
//     for (trace_id, list) in adjacency_lists {
//         if list.is_empty() {
//             continue;
//         }
//         println!("trace_id: {}", trace_id.0);
//         let l = &list[&SpanId::default()];
//         for (i, root) in l.iter().enumerate() {
//             dfs(&list, &spans_map, *root, 0, i == l.len() - 1);
//         }
//         println!();
//     }
// }
use mio::net::UdpSocket;
use std::any::type_name;
use std::net::SocketAddr;

// TODO : Read the cargo handbook to get some more insights
// on how to read crates.io docs
// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// @see https://www.youtube.com/watch?v=5LdnfzFdWhE --> understanding difference between packages, crates, mod for internal, use for external,
// @see https://www.youtube.com/watch?v=gi0AQ78diSA&t=224s --> to better understand how struct works
// @see https://docs.rs/mio/latest/mio/net/struct.UdpSocket.html --> this documentation explains how to construct a peer address for rust

fn main() -> Result() {
    // if you want to know if it was successful you can either use ? to propagate the error to the caller (if your function didn't return, the call succeeded), or use a match to explicitly handle both the success and error cases --> from discord server.
    let mut sender_socket = UdpSocket::bind("127.0.0.1:0".parse()?)?;
    let mut echoer_socket = UdpSocket::bind("127.0.0.1:0".parse()?)?;

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
}

/// pass by reference
/// when you pass parameters by reference
/// unlike value parameters, a new storage location is not created for these parameters
/// the reference parameters represent the same memory location as the actual parameters that are supplied to the method.
/// parameter values can be passed by reference by prefixing the variable name with an &
/// parameters can be immutable, since they generally remain unchanged once passed in once.
/// @see https://stackoverflow.com/questions/28255861/convert-string-to-socketaddr --> function explaining how the conversion logic works
fn convertToSocketAddr(server_string: &str) -> std::net::SocketAddr {
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
