use crate::config::Config;

#[derive(Debug, Default, Clone)]
pub struct Client {
    pub ctx: Config,
}

impl Client {
    /// create a new client instance
    pub fn new(config: Config) -> Client {
        Client { ctx: config }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_config() -> Config {
        Config::read_config("tests/server-config.toml").unwrap()
    }

    #[test]
    fn new() {
        let client = Client::new(create_config());
        print!("{:?}", client);

        assert!(true);
    }
}
