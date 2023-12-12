//
// the main driver
//
use anyhow::Result;
use clap::Parser;
use log::info;
use std::env;
use tiny_kv::db::DataStore;
use udp_socket_service::config::Config;
use udp_socket_service::handler::Handler;
use udp_socket_service::server::start;

#[derive(Debug, Default, Parser)]
#[command(
    name="udp-server",
    author,
    version,
    about="A udp server backed by tiny-kv handler.",
    long_about=None,
)]
struct Cli {
    /// config filename to override default
    #[arg(short, long, default_value_t = String::from("./config/server-config.toml"))]
    config_file: String,
}

fn create_handler(args: Vec<String>) -> Handler {
    let cli = Cli::parse_from(args);
    let config = Config::read_config(&cli.config_file).unwrap();
    let _ = config.start_logger();

    info!("cli: {:?}", cli);

    let db = DataStore::create();
    Handler::new(db)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    start(create_handler(args)).await
}

#[cfg(test)]
mod tests {
    use crate::create_handler;

    #[test]
    fn test_create_handler() {
        let args: Vec<String> = vec!["udp-server".to_string()];
        let handler = create_handler(args);
        assert_eq!(handler.db.dbsize(), 0);
    }
}
