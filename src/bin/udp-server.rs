//
// the main driver
//
use anyhow::Result;
use tiny_kv::db::DataStore;
use udp_socket_service::handler::Handler;
use udp_socket_service::server::start;

#[tokio::main]
async fn main() -> Result<()> {
    let db = DataStore::create();
    let handler = Handler::new(db);
    start(handler).await
}
