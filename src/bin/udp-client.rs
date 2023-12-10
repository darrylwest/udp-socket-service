use anyhow::{anyhow, Result};
use udp_socket_service::client::Client;
use udp_socket_service::config::Config;

fn create_client() -> Result<Client> {
    let filename = "./config/client-config.toml";
    match Config::read_config(filename) {
        Ok(config) => Ok(Client::new(config)),
        Err(e) => Err(anyhow!("could not read config: {}", e)),
    }
}

fn main() -> Result<()> {
    create_client()?.start()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_client_test() {
        let client = create_client();
        println!("{:?}", client);
        assert!(true);
    }
}
