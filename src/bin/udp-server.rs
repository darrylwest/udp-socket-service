//
// the main driver
//
use anyhow::Result;
use udp_socket_service::handler::Handler;
use udp_socket_service::server::start;

#[tokio::main]
async fn main() -> Result<()> {
    start(Handler {}).await
}
