use serde::Deserialize;
use serde_yaml;
use std::{error::Error, net::IpAddr};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
enum Variables {
    Client,
    Server,
    Port,
    MessageSize,
    BatchSize,
    NumberBatches,
    NumberClients,
    Role,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub enum Role {
    Server,
    Client,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    client: IpAddr,
    server: IpAddr,
    port: u16,
    message_size: usize,
    batch_size: usize,
    number_batches: usize,
    number_clients: usize,
    role: Role,
}

#[allow(unused)]
impl Config {
    pub fn read_config_file() -> Config {
        let config_file = std::fs::File::open("config.yaml").expect("Unable to open Config file");
        let config: Config =
            serde_yaml::from_reader(config_file).expect("Unable to read Config file");
        config
    }

    pub fn update(&mut self, variable: &str, value: Option<&str>) -> Result<(), Box<dyn Error>> {
        match variable {
            "client" | "Client" | "c" => match value {
                Some(value) => {
                    self.client = value
                        .parse::<IpAddr>()
                        .expect("Unable to retrieve server IP address from terminal")
                }
                None => return Err("Unknown Argument: client!")?,
            },
            "server" | "Server" | "s" => match value {
                Some(value) => {
                    self.server = value
                        .parse::<IpAddr>()
                        .expect("Unable to retrieve client's IP address from terminal")
                }
                None => return Err("Unknown Argument: server!")?,
            },
            "port" | "Port" | "p" => match value {
                Some(value) => {
                    self.port = value
                        .parse::<u16>()
                        .expect("Unable to retrieve port from terminal")
                }
                None => return Err("Unknown Argument: port!")?,
            },
            "messagesize" | "MessageSize" | "m" => match value {
                Some(value) => {
                    self.message_size = value
                        .parse::<usize>()
                        .expect("Unable to retrieve port from terminal")
                }
                None => return Err("Unknown Argument: message_size!")?,
            },
            "batchsize" | "BatchSize" | "b" => match value {
                Some(value) => {
                    self.batch_size = value
                        .parse::<usize>()
                        .expect("Unable to retrieve port from terminal")
                }
                None => return Err("Unknown Argument: batch_size!")?,
            },
            "numberbatches" | "NumberBatches" | "n" => match value {
                Some(value) => {
                    self.number_batches = value
                        .parse::<usize>()
                        .expect("Unable to retrieve port from terminal")
                }
                None => return Err("Unknown Argument: number_batches!")?,
            },
            "numberclients" | "numberClients" | "nc" => match value {
                Some(value) => {
                    self.number_clients = value
                        .parse::<usize>()
                        .expect("Unable to retrieve port from terminal")
                }
                None => return Err("Unknown Argument: number_clients!")?,
            },
            "role" | "Role" | "r" => match value {
                Some("client") | Some("Client") | Some("c") => self.role = Role::Server,
                Some("server") | Some("Server") | Some("s") => self.role = Role::Client,
                Some(_) | None => return Err("Unknown Argument: role!")?,
            },

            _ => return Err("Unknown Argument!")?,
        }
        Ok(())
    }

    pub fn client(&self) -> IpAddr {
        self.client
    }

    pub fn role(&self) -> Role {
        self.role
    }

    pub fn server(&self) -> IpAddr {
        self.server
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn message_size(&self) -> usize {
        self.message_size
    }

    pub fn batch_size(&self) -> usize {
        self.batch_size
    }

    pub fn number_batches(&self) -> usize {
        self.number_batches
    }

    pub fn number_clients(&self) -> usize {
        self.number_clients
    }

    pub fn usage() {
        println!("usage:");
        println!("======");
        println!("./benchmark_network --[Variable=Value]");
        println!("Variables:");
        for variable in Variables::iter() {
            println!("{:?}", variable);
        }
        println!("Example: ./benchmark_network --role=server --p=7777");
        println!("Example: cargo run -- --role=server --p=7777");
    }
}

#[cfg(test)]
mod test {

    use super::Config;

    #[test]
    #[ignore = "Config might Change"]
    fn test_read_config_file() {
        let config = Config::read_config_file();
        assert_eq!(config.server().to_string(), "127.0.0.1".to_string());
        assert_eq!(config.client().to_string(), "127.0.0.1".to_string());
        assert_eq!(config.port(), 7777);
        assert_eq!(config.message_size(), 10000);
        assert_eq!(config.batch_size(), 10000);
        assert_eq!(config.number_batches(), 1);
    }
}
