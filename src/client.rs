use crate::config::Config;
use anyhow::Result;
use log::info;
use std::io::{self, Write};
use std::net::UdpSocket;

#[derive(Debug, Default, Clone)]
pub struct Client {
    pub ctx: Config,
}

impl Client {
    /// create a new client instance
    pub fn new(config: Config) -> Client {
        Client { ctx: config }
    }

    /// get address from config
    fn create_server_addr(&self) -> String {
        format!("{}:{}", self.ctx.host, self.ctx.port)
    }

    /// start the client repl
    pub fn start(&self) -> Result<()> {
        // explain this...
        let socket = UdpSocket::bind("127.0.0.1:0")?;
        socket.set_write_timeout(Some(std::time::Duration::new(5, 0)))?;
        socket.set_read_timeout(Some(std::time::Duration::new(5, 0)))?;

        let server_address = self.create_server_addr();
        let mut ln = 0;

        info!("Enter 'quit' or ^c to exit...");
        loop {
            ln += 1;
            info!("{} > ", ln);
            let _ = io::stdout().flush();
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            if input.starts_with("quit") {
                break;
            }

            let message = input.as_bytes();
            socket.send_to(message, server_address.as_str())?;

            let mut buffer = [0; 1024];
            let (amt, _) = socket.recv_from(&mut buffer)?;
            info!("{}", String::from_utf8_lossy(&buffer[..amt]));
        }

        Ok(())
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
        info!("{:?}", client);

        assert!(true);
    }
}
