use super::util::to_string;
use std::io::Result;
use std::net::{IpAddr, TcpStream};

/// Connects the client to the designated server on a specific port
///
/// # Example
/// ```
/// # use benchmark_network::{client, io::send, server};
/// # use std::net::{IpAddr, Ipv4Addr};
/// # const LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
/// # const SOURCE: IpAddr = LOCALHOST;
/// # const PORT: u16 = 6666;
/// let mut stream = client::connect(LOCALHOST, PORT);
/// assert!(stream.is_ok());
/// ```
/// In order to run this example, a server should be running first.
pub fn connect(destination: IpAddr, port: u16) -> Result<TcpStream> {
    let host = to_string(destination, port);
    TcpStream::connect(host)
}

/// Unimplemented yet
#[allow(unused_variables)]
pub fn disconnect(destination: IpAddr, port: u16) -> Result<()> {
    //TODO: To be implemented
    Ok(())
}
