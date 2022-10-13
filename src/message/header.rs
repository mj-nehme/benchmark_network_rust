use super::{mtype::Type, Byte};
use std::{mem, net::IpAddr};
use uuid::Uuid;

/// Header includes the message metadata: source, destination, type, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    id: Uuid,
    source: IpAddr,
    destination: IpAddr,
    mtype: Type,
    // Maximum size 4_294_967_295usize
    body_size: usize,
}

impl Header {
    /// Creates a new Header given the source and destination IP addresses,
    /// the body size and the message type (Propose, Acknowledge...).
    pub fn new(source: IpAddr, destination: IpAddr, body_size: usize, mtype: Type) -> Header {
        let id = Uuid::new_v4();
        Self {
            id,
            source,
            destination,
            body_size,
            mtype,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn source(&self) -> IpAddr {
        self.source
    }

    pub fn destination(&self) -> IpAddr {
        self.destination
    }

    pub fn mtype(&self) -> Type {
        self.mtype
    }

    pub fn body_size(&self) -> usize {
        self.body_size
    }

    // TODO: For now, Header works on homogeneous architectures only
    // Handle heterogeneous by specifying the size of each struct element.
    // If some architecture for example considers the size of IpAddr
    // different from another, this leads to inconsistency.
    pub const fn size() -> usize {
        mem::size_of::<Header>()
    }

    pub unsafe fn to_bytes(&self) -> &[Byte] {
        ::std::slice::from_raw_parts((self as *const Header) as *const u8, Header::size())
    }

    pub unsafe fn to_mut_bytes(&mut self) -> &mut [Byte] {
        ::std::slice::from_raw_parts_mut((self as *mut Header) as *mut u8, Header::size())
    }

    pub unsafe fn from_bytes(bytes: &[Byte]) -> Header {
        let p: *const [u8; Header::size()] = bytes.as_ptr() as *const [u8; Header::size()];
        std::mem::transmute(*p)
    }

    pub fn print(&self) {
        println!("message ID: {}", self.id);
        println!("source IP: {}", self.source);
        println!("destination IP: {}", self.destination);
        println!("body_size: {}", self.body_size);
        println!("message type: {:?}", self.mtype);
    }
}
