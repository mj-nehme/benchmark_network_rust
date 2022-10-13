mod tests {
    use crate::message::{message::Message, mtype::Type};
    use crate::network::{client, io::send, server};
    use std::net::{IpAddr, Ipv4Addr};

    const LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    const SOURCE: IpAddr = LOCALHOST;
    const PORT: u16 = 6666;
    const DESTINATION: IpAddr = LOCALHOST;
    const BODY_SIZE: usize = 10;
    const MTYPE: Type = Type::Benchmark;

    #[ignore = "runs indefinitley"]
    #[test]
    pub fn test_server() {
        server::listen(PORT, 1, 1);
    }

    #[ignore = "might run before server"]
    #[test]
    pub fn test_client() {
        let mut stream = client::connect(LOCALHOST, PORT).expect("Unable to connect");
        assert_eq!(stream.peer_addr().unwrap().ip(), LOCALHOST);

        // Message with body
        let msg = Message::new(SOURCE, DESTINATION, BODY_SIZE, MTYPE);
        let size = send(&mut stream, &msg).expect("Unable to send message");
        assert_eq!(size, BODY_SIZE);

        // Message without body
        let close_message = Message::new(SOURCE, DESTINATION, 0, Type::Close);
        let size = send(&mut stream, &close_message).expect("Unable to send message");
        assert_eq!(size, 0);
    }
}
