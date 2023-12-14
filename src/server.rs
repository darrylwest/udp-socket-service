//
//
//

use crate::config::Config;
use crate::handler::{Handler, Request, Response, Status};
use anyhow::Result;
use log::info;
use tokio::net::UdpSocket;

#[derive(Debug, Default, Clone)]
pub struct Server {
    config: Config,
    handler: Handler,
}

impl Server {
    /// create the server from config and handler
    pub fn create(config: Config, handler: Handler) -> Server {
        Server { config, handler }
    }

    fn create_addr(&self) -> String {
        format!("{}:{}", self.config.host, self.config.port)
    }

    /// pull out the handler
    pub async fn start(&mut self) -> Result<()> {
        let addr = self.create_addr();
        info!("listening on: {}", addr);

        let sock = UdpSocket::bind(addr).await?;

        loop {
            // listen for a message
            let mut buf = [0; 128];
            info!("wait for a connection...");

            let (len, addr) = sock.recv_from(&mut buf).await?;
            let msg = String::from_utf8_lossy(&buf[..len]);
            let msg = msg.trim();

            info!("recv: {} bytes from {:?}, msg: {}", len, addr, msg);
            // split this into [cmd, param, param]
            let response = match Request::from_message(msg) {
                Ok(request) => self.handler.handle_request(request),
                Err(e) => Response::create(Status::bad_request(), e.to_string()),
            };

            // return the response
            let resp = response.as_string();
            let len = sock.send_to(resp.as_bytes(), addr).await?;
            info!("returned: {:?}, size {}.", response, len);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tiny_kv::db::DataStore;

    fn create_config() -> Config {
        Config::read_config("./tests/server-config.toml").unwrap()
    }

    fn create_db() -> DataStore {
        DataStore::create()
    }

    fn create_server() -> Server {
        let config = create_config();
        let handler = Handler::new(create_db());
        Server::create(config, handler)
    }

    #[test]
    fn test_create() {
        let config = create_config();
        let handler = Handler::new(create_db());
        let server = Server::create(config, handler);

        println!("{:?}", server);
    }

    #[test]
    fn test_addr() {
        let server = create_server();
        let addr = server.create_addr();
        assert_eq!(addr, "127.0.0.1:28400");
    }
}
