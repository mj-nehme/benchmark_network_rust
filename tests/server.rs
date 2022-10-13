mod server {
    use benchmark_network::{config::Config, run_server};

    #[test]
    #[ignore = "runs indefinitely"]
    pub fn test() {
        let config: Config = Config::read_config_file();
        let number_of_clients = config.number_clients();
        run_server(config, Some(number_of_clients));
    }
}
