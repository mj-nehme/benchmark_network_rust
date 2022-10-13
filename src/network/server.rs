use super::{io, ThreadPool};
use crate::message::mtype::Type;
use std::net::{Shutdown, TcpListener, TcpStream};

const LOCALHOST: &str = "0.0.0.0";

/// Similar to listen, with a difference that it might listen to one (or) more clients
/// and stop listening.
/// It listens locally on the given port, and reads incoming messages
/// TODO: Add the handle_client function as parameter
pub fn listen_portable(port: u16, pool_size: usize, queue_size: usize, is_limited: Option<usize>) {
    assert!(queue_size <= pool_size);
    let address = format!("{}:{}", LOCALHOST, port.to_string());
    let listener = TcpListener::bind(address).unwrap();

    let pool = ThreadPool::new(pool_size);

    println!("Server listening on {}", port);

    let mut counter = is_limited.unwrap_or_default();

    for stream in listener.incoming().take(queue_size) {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());

                pool.execute(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }

        if is_limited.is_some() {
            counter -= 1;
            if counter == 0 {
                break;
            }
        }
    }

    println!("Dropping listener");
    drop(listener);
}

/// listens locally on the given port, and reads incoming messages
pub fn listen(port: u16, pool_size: usize, queue_size: usize) {
    listen_portable(port, pool_size, queue_size, None);
}

pub fn handle_client(mut stream: TcpStream) {
    let peer_address = stream.peer_addr().unwrap();
    loop {
        let message = match io::read(&mut stream) {
            Ok(message) => match message.mtype() {
                Type::Close => {
                    break;
                }
                _ => message,
            },
            Err(e) => {
                println!(
                    "An error occurred, terminating connection with {}\n {}",
                    peer_address, e
                );
                stream.shutdown(Shutdown::Both).unwrap();
                break;
            }
        };
        if message.has_body() {
            assert!(message.body_size() > 0);
        } else {
            assert_eq!(message.body_size(), 0);
        }
    }

    close(stream);
}

pub fn close(stream: TcpStream) {
    let peer_addr = stream
        .peer_addr()
        .expect("Unable to retrieve client's address on closing socket");
    println!("Closed connection to {}", peer_addr);
}
