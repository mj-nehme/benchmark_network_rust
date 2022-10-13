use std::net::IpAddr;

pub fn to_string(host: IpAddr, port: u16) -> String {
    format!("{}:{}", host.to_string(), port.to_string())
}
