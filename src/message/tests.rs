mod tests {
    use crate::message::{header::Header, message::Message, mtype::Type};
    use std::{
        net::{IpAddr, Ipv4Addr},
        ops::DerefMut,
    };

    type Byte = u8;
    const LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    const SOURCE: IpAddr = LOCALHOST;
    const DESTINATION: IpAddr = LOCALHOST;
    const BODY_SIZE: usize = 5;
    const MTYPE: Type = Type::Benchmark;
    const EMPTY_BYTE: Byte = u8::MIN;
    const FULL_BYTE: Byte = u8::MAX;

    #[test]
    fn test_new_message() {
        let msg = Message::new(SOURCE, DESTINATION, BODY_SIZE, MTYPE);
        assert!(msg.has_body());
        assert!(!msg.id().is_nil());
        assert_eq!(msg.source(), SOURCE);
        assert_eq!(msg.destination(), DESTINATION);
        assert_eq!(msg.body_size(), BODY_SIZE);
        assert_eq!(msg.mtype(), MTYPE);
        assert_eq!((**msg.body().unwrap()).len(), BODY_SIZE);
    }

    #[test]
    fn test_clone_message() {
        let mut msg = Message::new(SOURCE, DESTINATION, BODY_SIZE, MTYPE);

        // message clone
        let msg_clone = msg.clone();
        assert_eq!(msg, msg_clone);

        // Modify msg
        let vector = msg.body_as_mut().unwrap().deref_mut();
        let first_byte = &mut vector[0];
        *first_byte = 1;
        assert_ne!(msg, msg_clone);

        // clone is making a deep copy with Box
        assert_eq!(**(msg_clone.body().unwrap()), vec![EMPTY_BYTE; BODY_SIZE]);
    }

    #[test]
    fn test_message_serialization() {
        // Serialize and Deserialize Header
        let msg = Message::new(SOURCE, DESTINATION, BODY_SIZE, MTYPE);
        let header = msg.header();
        let bytes = unsafe { header.to_bytes() };
        assert_eq!(bytes[25..29], [127u8, 0u8, 0u8, 1u8]);
        let deserialized_header = unsafe { Header::from_bytes(&bytes) };
        assert_eq!(header, deserialized_header);

        // Serialize, modify source IP and Deserialize
        let msg = Message::new(SOURCE, DESTINATION, BODY_SIZE, MTYPE);
        let mut header = msg.header();
        let bytes = unsafe { header.to_mut_bytes() };
        assert_eq!(bytes[25..29], [127u8, 0u8, 0u8, 1u8]);
        bytes[28] = FULL_BYTE;
        let deserialized_header = unsafe { Header::from_bytes(&bytes) };
        assert_eq!(header, deserialized_header);
        assert_eq!(deserialized_header.source().to_string(), "127.0.0.255");
    }
}
