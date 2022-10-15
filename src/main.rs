use benchmark_network::{
    config::{Config, Role},
    run_client, run_server,
};
use std::{env, error::Error};
pub mod config;
pub mod unit;

fn main() {
    let args: Vec<String> = env::args().collect();
    let result = retrieve_args(args);
    match result {
        Ok(config) => execute(config),
        Err(e) => {
            println!("{e}");
            Config::usage();
        }
    }
}

/// Retrieves console arguments
/// Each argument should start with `--`
/// Arguments might be of one of two forms
/// 1. `--arg` such as `--help`
/// 2. `--arg=value` such as `--message-size=10000`
pub fn retrieve_args(args: Vec<String>) -> Result<Config, Box<dyn Error>> {
    let mut config: Config = Config::read_config_file();

    if args.len() == 1 {
        // If no arguments given, show usage
        return Err("No arguments provided!")?;
    }

    // Skip the first arg (the app name)
    let args = args[1..].to_vec();
    for arg in args {
        // Expecting "arg" to be something like --message-size=10000
        if arg.starts_with("--") {
            // Ignore --
            let (_, arg) = arg.split_at(2);
            let result: Vec<&str> = arg.split(|c| c == '=').collect();
            match result.len() {
                1 => config.parse_attributes(result[0], None),
                2 => config.parse_attributes(result[0], Some(result[1])),
                _ => return Err("Unknown Argument: more than one assignment!")?,
            }?;
            config.print();
        } else {
            return Err("Unknown Argument: args should start with --!")?;
        }
    }

    Ok(config)
}

pub fn execute(config: Config) {
    match config.role() {
        Role::Client => run_client(config),
        Role::Server => {
            let number_of_clients = config.number_clients();
            run_server(config, Some(number_of_clients))
        }
    }
}
