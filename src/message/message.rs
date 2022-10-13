use super::{body::Body, header::Header, mtype::Type, Byte, EMPTY_BYTE};
use std::{fmt::Debug, net::IpAddr};
use uuid::Uuid;

/// message crate is a network message used to be sent among nodes
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    header: Header,
    body: Body,
}

impl Message {
    /// Creates a new message given the source IP address, the destination IP
    /// address, the body size, the message type (Propose, Acknowledge...) and
    /// a message body (if exists).
    pub fn new(source: IpAddr, destination: IpAddr, body_size: usize, mtype: Type) -> Message {
        let header = Header::new(source, destination, body_size, mtype);
        let body = match body_size {
            0 => None,
            _ => {
                // The created message body is initially filled with
                // zeros. That's because a message is intended to be
                // used for benchmarking only.
                let content = vec![EMPTY_BYTE as Byte; body_size];
                let body = Box::new(content);
                Some(body)
            }
        };
        Self { header, body }
    }

    /// Creates a new message given the header and the body
    pub fn compose(header: Header, body: Option<Box<Vec<Byte>>>) -> Message {
        Self { header, body }
    }

    /// Returns the message Header
    pub fn header(&self) -> Header {
        self.header
    }

    /// Returns the message id
    pub fn id(&self) -> Uuid {
        self.header.id()
    }

    /// Returns the message source IpAddr
    pub fn source(&self) -> IpAddr {
        self.header.source()
    }

    /// Returns the message destination IpAddr
    pub fn destination(&self) -> IpAddr {
        self.header.destination()
    }

    /// Returns the message Type (Propose, Acknowledge...)
    pub fn mtype(&self) -> Type {
        self.header.mtype()
    }

    /// Returns the body size of a message, not the entire message size
    pub fn body_size(&self) -> usize {
        self.header.body_size()
    }

    /// Returns the message body. This is an optional field of the message
    pub fn body(&self) -> Option<&Box<Vec<Byte>>> {
        self.body.as_ref()
    }

    /// Returns the message body as mutable. This is an optional field of the message
    pub fn body_as_mut(&mut self) -> Option<&mut Box<Vec<Byte>>> {
        self.body.as_mut()
    }

    /// Checks if a message has a body or only a header
    pub fn has_body(&self) -> bool {
        matches!(self.body, Some(_))
    }

    /// Checks if the message has no body (only a header)
    pub fn without_body(&self) -> bool {
        matches!(self.body, None)
    }

    /// Returns the Header size
    pub fn header_size() -> usize {
        Header::size()
    }

    /// Prints the message header. Used for debugging purposes
    pub fn print(&self) {
        self.header.print();
    }
}
