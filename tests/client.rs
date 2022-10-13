mod client {
    use benchmark_network::{config::Config, run_client};

    #[ignore = "might run before server"]
    #[test]
    pub fn test() {
        let config: Config = Config::read_config_file();
        run_client(config)
    }
}
