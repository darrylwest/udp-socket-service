///
/// the main client repl
///
use anyhow::{anyhow, Result};
use clap::Parser;
use std::env;
use std::net::UdpSocket;
use udp_socket_service::config::Config;

#[derive(Debug, Clone)]
pub struct RequestClient {
    ctx: Config,
}

impl RequestClient {
    pub fn new(config: Config) -> RequestClient {
        RequestClient { ctx: config }
    }

    /// get address from config
    fn create_server_addr(&self) -> String {
        let server_address = format!("{}:{}", self.ctx.host, self.ctx.port);
        server_address
    }

    // create the socket and set the timeout values
    fn create_socket(&self) -> Result<UdpSocket> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_write_timeout(Some(std::time::Duration::new(3, 0)))?;
        socket.set_read_timeout(Some(std::time::Duration::new(3, 0)))?;

        Ok(socket)
    }

    // open the socket and send the request
    pub fn send_request(&self, message: &str) -> Result<String> {
        let server_address = self.create_server_addr();
        let socket = self.create_socket()?;

        socket.send_to(message.as_bytes(), server_address.as_str())?;

        let mut buffer = [0; 1024];
        let (sz, _) = socket.recv_from(&mut buffer)?;
        let response = String::from_utf8_lossy(&buffer[..sz]).to_string();

        Ok(response)
    }
}

#[derive(Debug, Default, Parser)]
#[command(
    name="udp-client",
    author,
    version,
    about="A UDP request client for udp-server & k/v store.",
    long_about=None,
)]
struct Cli {
    /// config filename to override default
    #[arg(short, long, default_value_t = String::from("./config/client-config.toml"))]
    config_file: String,

    /// send a request message; default is status
    #[arg(short, long, default_value_t = String::from("status"))]
    message: String,
}

/// create the repl client from args return the client or error
fn send_request(args: Vec<String>) -> Result<()> {
    let cli = Cli::parse_from(args);

    match Config::read_config(&cli.config_file) {
        Ok(config) => {
            // let _ = config.start_logger();
            let client = RequestClient::new(config);
            let response = client.send_request(cli.message.as_str())?;
            println!("{}", response);
            Ok(())
        }
        Err(e) => Err(anyhow!("could not read config: {}", e)),
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let _ = send_request(args);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_client_test() {
        let args: Vec<String> = vec!["udp-request".to_string()];
        let client = send_request(args);
        println!("{:?}", client);
        assert!(true);
    }
}
