use crate::config::Config;
use anyhow::Result;
use std::io::{self, Write};
use std::net::UdpSocket;

#[derive(Debug, Default, Clone)]
pub struct Client {
    pub ctx: Config,
}

fn read_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}

impl Client {
    /// create a new client instance
    pub fn new(config: Config) -> Client {
        Client { ctx: config }
    }

    /// get address from config
    fn create_server_addr(&self) -> String {
        let server_address = format!("{}:{}", self.ctx.host, self.ctx.port);
        println!("listen on addr: {}", server_address);
        server_address
    }

    fn create_socket(&self) -> Result<UdpSocket> {
        // explain this...
        let socket = UdpSocket::bind("127.0.0.1:0")?;
        socket.set_write_timeout(Some(std::time::Duration::new(5, 0)))?;
        socket.set_read_timeout(Some(std::time::Duration::new(5, 0)))?;

        Ok(socket)
    }

    fn start_repl(&self, socket: UdpSocket, server_address: &str) -> Result<()> {
        println!("Enter 'quit' or ^c to exit...");
        let mut ln = 0;
        loop {
            ln += 1;
            print!("{} > ", ln);
            let _ = io::stdout().flush();

            let input = read_input();
            if input.starts_with("quit") {
                break;
            }

            let message = input.as_bytes();
            socket.send_to(message, server_address)?;

            let mut buffer = [0; 1024];
            let (amt, _) = socket.recv_from(&mut buffer)?;
            println!("{}", String::from_utf8_lossy(&buffer[..amt]));
        }

        Ok(())
    }

    /// start the client repl
    pub fn start(&self) -> Result<()> {
        let socket = self.create_socket()?;
        let server_address = self.create_server_addr();

        self.start_repl(socket, server_address.as_str())
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
        println!("{:?}", client);

        assert!(true);
    }

    #[test]
    fn create_addr() {
        let client = Client::new(create_config());
        let addr = client.create_server_addr();
        println!("addr: {}", addr);
        assert_eq!(addr, "127.0.0.1:28400");
    }

    #[test]
    fn create_socket() {
        let client = Client::new(create_config());
        let socket = client.create_socket().unwrap();
        println!("{:?}", socket);
    }
}
