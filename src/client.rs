use crate::config::Config;
use anyhow::Result;
use std::io::{self, Write};
use std::net::UdpSocket;

#[derive(Debug, Clone)]
pub struct Client {
    ctx: Config,
    prompter: fn(line_num: usize, prompt: &str) -> String,
}

/// show the repl prompt, line number and >
fn show_prompt(ln: usize, prompt: &str) {
    print!("{}{} ", ln, prompt);
    let _ = io::stdout().flush();
}

/// read the next repl command from stdin
fn read_input(ln: usize, prompt: &str) -> String {
    show_prompt(ln, prompt);
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}

impl Client {
    /// create a new client instance
    pub fn new(config: Config) -> Client {
        Client {
            ctx: config,
            prompter: read_input,
        }
    }

    /// get address from config
    fn create_server_addr(&self) -> String {
        let server_address = format!("{}:{}", self.ctx.host, self.ctx.port);
        println!("listen on addr: {}", server_address);
        server_address
    }

    // create the socket and set the timeout values
    fn create_socket(&self) -> Result<UdpSocket> {
        let socket = UdpSocket::bind("127.0.0.1:0")?;
        socket.set_write_timeout(Some(std::time::Duration::new(5, 0)))?;
        socket.set_read_timeout(Some(std::time::Duration::new(5, 0)))?;

        Ok(socket)
    }

    /// start the repl loop
    fn start_repl(&self, socket: UdpSocket, server_address: &str) -> Result<()> {
        println!("Enter 'quit' or ^c to exit...");
        let mut ln = 0;
        loop {
            ln += 1;

            let input = (self.prompter)(ln, " >");
            let message = input.as_bytes();

            if input.starts_with("quit") {
                break;
            }

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

    fn mock_reader(ln: usize, prompt: &str) -> String {
        println!("{}", prompt);

        match ln {
            1 => "quit".to_string(),
            _ => "quit".to_string(),
        }
    }

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

    #[test]
    fn start_repl() {
        // the server needs to be up for this to work...
        let reader = mock_reader;
        assert_eq!(reader(1, "anything"), "quit");
        assert_eq!(reader(9, "> "), "quit");

        let client = Client {
            ctx: create_config(),
            prompter: mock_reader,
        };

        let socket = client.create_socket().unwrap();
        let server_address = client.create_server_addr();

        let resp = client.start_repl(socket, server_address.as_str());
        assert!(resp.is_ok());
    }
}
