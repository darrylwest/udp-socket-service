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

    async fn bind_socket(&self) -> Result<UdpSocket> {
        let addr = format!("{}:{}", self.config.host, self.config.port);
        info!("listening on: {}", addr);
        let sock = UdpSocket::bind(addr).await?;

        Ok(sock)
    }

    /// pull out the handler
    pub async fn start(&mut self) -> Result<()> {
        let sock = self.bind_socket().await.expect("open socket error");

        loop {
            // listen for a message
            let mut buf = [0; 128];

            let (len, addr) = sock.recv_from(&mut buf).await?;
            let msg = String::from_utf8_lossy(&buf[..len]);
            let msg = msg.trim();

            info!("recv: {} bytes from {:?}, msg: {}", len, addr, msg);

            if msg == "shutdown" {
                info!("{}", "received shutdown command");
                Config::remove_pid_file();
                break;
            }

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

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tiny_kv::db::DataStore;
    // use tokio_test::*;

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

    #[tokio::test]
    async fn start() {
        let ctx = create_config();
        let config = Config {
            name: ctx.name.to_string(),
            version: ctx.version.to_string(),
            host: ctx.host.to_string(),
            port: 9898,
            logging_config: ctx.logging_config.to_string(),
            data_file: ctx.data_file.clone(),
        };

        let handler = Handler::new(create_db());
        let mut server = Server::create(config.clone(), handler);

        let addr = format!("{}:{}", config.clone().host, config.clone().port);
        let client = UdpSocket::bind("127.0.0.1:0").await.unwrap();

        let server_task = tokio::spawn(async move {
            let result = server.start().await;
            println!("{:?}", result);
        });

        let client_task = tokio::spawn(async move {
            let result = client.send_to(b"shutdown", addr).await;
            println!("{:?}", result);
        });

        client_task.await.unwrap();
        server_task.await.unwrap();
    }

    #[tokio::test]
    async fn bind_socket() {
        let server = create_server();
        let sock = server.bind_socket().await;
        println!("{:?}", sock);
    }

    #[test]
    fn test_create() {
        let config = create_config();
        let handler = Handler::new(create_db());
        let server = Server::create(config, handler);

        println!("{:?}", server);
    }
}
