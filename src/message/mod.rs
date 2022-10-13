pub mod body;
pub mod header;
pub mod message;
pub mod mtype;

pub type Byte = u8;
pub const EMPTY_BYTE: Byte = u8::MIN;

#[cfg(test)]
mod tests;
