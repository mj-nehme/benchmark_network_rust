use crate::message::{header::Header, message::Message, Byte};
use std::error::Error;
use std::io::{Read, Write};
use std::net::TcpStream;

/// Writes a Message ot the buffer. The message is written in two steps.
///  1. The message is serialized into a buffer. The buffer size is
///     calculated and sent to the receiver.
///  2. The buffer itself is sent. A maximum buffer size is supposed to
///     be no more than u128
///
/// # Example
/// ```
/// # use benchmark_network::{client, io::send, server};
/// # use benchmark_network::{message::Message, mtype::Type};
/// # use std::net::{IpAddr, Ipv4Addr};
/// #
/// # const LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
/// # const SOURCE: IpAddr = LOCALHOST;
/// # const DESTINATION: IpAddr = LOCALHOST;
/// # const PORT: u16 = 6666;
/// # const BODY_SIZE: usize = 10;
/// # const MTYPE: Type = Type::Benchmark;
/// # const EMPTY_BYTE: u8 = u8::MIN;
/// #
/// # let mut stream = client::connect(LOCALHOST, PORT).expect("Unable to connect");
/// # assert_eq!(stream.peer_addr().unwrap().ip(), LOCALHOST);
/// #
/// # let content = vec![EMPTY_BYTE as u8; BODY_SIZE];
/// # let body = Box::new(content);
/// # let msg = Message::<Vec<u8>>::new(SOURCE, DESTINATION, BODY_SIZE, MTYPE, body.into());
/// # let size = send(&mut stream, &msg).expect("Unable to send message");
/// # assert_eq!(size, 61 + BODY_SIZE as u128);
/// #
/// # let msg = Message::<Vec<u8>>::new(SOURCE, DESTINATION, 0, Type::Close, None);
///   let size = send(&mut stream, &msg).expect("Unable to send message");
///   assert_eq!(size, 53);
/// ```
/// In order to run this example, a server should be running first.
pub fn send(stream: &mut TcpStream, msg: &Message) -> Result<usize, Box<dyn Error>> {
    // Write buffer size so that the other end expects how much bytes to read.
    unsafe { stream.write(msg.header().to_bytes())? };

    // Write buffer
    match msg.body() {
        Some(body) => Ok(stream.write(body)?),
        None => Ok(0),
    }
}

/// Reads Result<Message> from a TcpStream
///
/// # Example
/// ```
/// # use benchmark_network::{client, io::read, server::{self, close}};
/// # use benchmark_network::{message::Message, mtype::Type};
/// # use std::thread;
/// # use std::net::{Shutdown, TcpListener, TcpStream, IpAddr, Ipv4Addr};
/// # use serde::de::DeserializeOwned;
/// #
/// # const NOADDRESS: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
/// # const PORT: u16 = 6666;
/// # const EMPTY_BYTE: u8 = u8::MIN;
/// #
/// # fn main() -> std::io::Result<()> {
/// #     let address = format!("{}:{}", NOADDRESS, PORT.to_string());
/// #     let listener = TcpListener::bind(address).unwrap();
/// #     println!("Server listening on port {}", PORT);
/// #     for stream in listener.incoming() {
/// #         match stream {
/// #             Ok(stream) => {
/// #                 println!("New connection: {}", stream.peer_addr().unwrap());
/// #                 let handle = thread::spawn(move || handle_client::<Vec<u8>>(stream));
/// #             }
/// #             Err(e) => {
/// #                 println!("Connection failed: {}", e);
/// #             }
/// #         }
/// #     }
/// #     Ok(())
/// # }
/// #
/// # pub fn handle_client<T: Send + Sync + DeserializeOwned>(mut stream: TcpStream) {
/// #     loop {
///          let message = match read::<T>(&mut stream) {
///              Ok(message) => match message.mtype() {
/// #                 Type::Close => {
/// #                     break;
/// #                 }
/// #                 _ => {
/// #                     println!("Read a message");
///                       message
/// #                 }
///               },
///               Err(e) => {
/// #                 println!(
/// #                     "An error occurred, terminating connection with {}\n {}",
/// #                     stream.peer_addr().unwrap(),
/// #                     e
/// #                 );
/// #                 stream.shutdown(Shutdown::Both).unwrap();
///                  break;
///               }
///           };
/// #         if message.has_body() {
/// #             assert!(message.body_size() > 0);
/// #         } else {
/// #             assert_eq!(message.body_size(), 0);
/// #         }
/// #     }
/// #     close(stream);
/// # }
/// ```
pub fn read(stream: &mut TcpStream) -> Result<Message, Box<dyn Error>> {
    // Read expected message header
    let mut header_buffer = [0u8; Header::size()];
    stream.read_exact(&mut header_buffer)?;
    let header = unsafe { Header::from_bytes(&header_buffer) };

    // Read message body
    let body: Option<Box<Vec<Byte>>> = match header.body_size() {
        0 => None,
        _ => {
            let mut buffer: Vec<Byte> = vec![0u8; header.body_size() as usize];
            stream.read_exact(&mut buffer)?;
            Some(Box::new(buffer))
        }
    };

    Ok(Message::compose(header, body))
}
