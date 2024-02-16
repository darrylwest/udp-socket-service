///
/// the main client repl
///
use anyhow::{anyhow, Result};
use clap::Parser;
use std::env;
use udp_socket_service::client::Client;
use udp_socket_service::config::Config;

#[derive(Debug, Default, Parser)]
#[command(
    name="udp-client",
    author,
    version,
    about="A repl client for udp-server backed by tiny-kv.",
    long_about=None,
)]
struct Cli {
    /// config filename to override default
    #[arg(short, long, default_value_t = String::from(".config/udp-config/client-config.toml"))]
    config_file: String,
}

/// create the repl client from args return the client or error
fn create_client(args: Vec<String>) -> Result<Client> {
    let cli = Cli::parse_from(args);

    match Config::read_config(&cli.config_file) {
        Ok(config) => {
            let _ = config.start_logger();
            Ok(Client::new(config))
        }
        Err(e) => Err(anyhow!("could not read config: {}", e)),
    }
}

fn main() -> Result<()> {
    let home = env::var_os("HOME").unwrap();
    env::set_current_dir(home).unwrap();

    let args: Vec<String> = env::args().collect();
    create_client(args)?.start()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_client_test() {
        let args: Vec<String> = vec!["udp-client".to_string()];
        let client = create_client(args);
        println!("{:?}", client);
        assert!(true);
    }

    #[test]
    fn create_client_bad() {
        let args: Vec<String> = vec![
            "udp-client".to_string(),
            "--config-file".to_string(),
            "/bad/file".to_string(),
        ];
        let client = create_client(args);
        println!("{:?}", client);
        assert!(true);
    }
}
