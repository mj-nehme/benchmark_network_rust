use crate::config::Config;
pub mod config;
pub mod message;
pub mod network;
pub mod unit;
use crate::{
    message::{message::Message, mtype::Type},
    network::{client, io::send},
    unit::*,
};
use network::server;
use std::time::Instant;

pub fn run_server(config: Config, is_limited: Option<usize>) {
    let port = config.port();
    let pool_size = 1;
    let queue_size = 1;
    server::listen_portable(port, pool_size, queue_size, is_limited);
}

pub fn run_client(config: Config) {
    let server_address = config.server();
    let port = config.port();
    println!("Connecting to {}:{}...", server_address, port);
    let mut stream = client::connect(server_address, port).expect("Unable to connect");
    assert_eq!(stream.peer_addr().unwrap().ip(), server_address);

    let client_address = config.client();
    let body_size = config.message_size();
    let mtype = Type::Benchmark;
    let msg = Message::new(client_address, server_address, body_size, mtype);
    let mut sent_batches = 0;
    loop {
        let now = Instant::now();

        for _counter in 0..config.batch_size() {
            let size = send(&mut stream, &msg).expect("Unable to send message");
            assert_eq!(size, body_size);
        }

        let elapsed = now.elapsed().as_secs_f64();
        let sent_bytes = (config.message_size() * config.batch_size()) as f64;
        let sent_bits = (sent_bytes * 8.0) as f64;

        let throughput = sent_bits / elapsed;
        println!(
            "Throughput: {}; Sent bytes: {}; Time elapsed: {:.2}s",
            throughput_to_string(throughput),
            data_to_string(sent_bytes),
            elapsed
        );
        sent_batches += 1;
        if sent_batches == config.number_batches() {
            break;
        }
    }

    let msg = Message::new(client_address, server_address, 0, Type::Close);
    let size = send(&mut stream, &msg).expect("Unable to send message");
    assert_eq!(size, 0);
}
