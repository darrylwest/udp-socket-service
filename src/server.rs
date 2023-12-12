//
//
//

use crate::handler::{Handler, Request, Response, Status};
use anyhow::Result;
use log::info;
use tokio::net::UdpSocket;

/// pull out the handler
pub async fn start(mut handler: Handler) -> Result<()> {
    let addr = "0.0.0.0:22200";
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
            Ok(request) => handler.handle_request(request),
            Err(e) => Response::create(Status::bad_request(), e.to_string()),
        };

        // return the response
        let resp = response.as_string();
        let len = sock.send_to(resp.as_bytes(), addr).await?;
        info!("returned: {:?}, size {}.", response, len);
    }
}
