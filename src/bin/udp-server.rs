//
// the main driver
//
use anyhow::Result;
use clap::Parser;
use log::{error, info};
use std::env;
use tiny_kv::db::DataStore;
use udp_socket_service::config::Config;
use udp_socket_service::handler::Handler;
use udp_socket_service::server::Server;

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

    /// an optional data file to load on startup.  this overrides the file in config
    #[arg(short, long)]
    data_file: Option<String>,
}

/// create the default handler
fn create_handler(datafile: Option<String>) -> Handler {
    let db = DataStore::create();
    if datafile.is_some() {
        let filename = datafile.unwrap();
        info!("load data from: {}", filename);
        match db.loaddb(&filename) {
            Ok(sz) => info!("data loaded, {} elements...", sz),
            Err(e) => {
                let msg = format!("error loading data from {}, {}", filename, e);
                error!("{}", msg);
            }
        }
    }

    Handler::new(db)
}

/// create the udp server
fn create_server(args: Vec<String>) -> Result<Server> {
    let cli = Cli::parse_from(args);
    let config = Config::read_config(&cli.config_file).unwrap();
    Config::write_pid_file();

    let _ = config.start_logger();

    info!("cli: {:?}", cli);

    let mut datafile: Option<String> = None;
    if cli.data_file.is_some() {
        datafile = cli.data_file;
    } else if config.data_file.is_some() {
        datafile = config.data_file.clone();
    }

    let handler = create_handler(datafile);
    let server = Server::create(config.clone(), handler);

    Ok(server)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    create_server(args)?.start().await
}

#[cfg(test)]
mod tests {
    use crate::create_handler;

    #[test]
    fn test_create_handler() {
        // let args: Vec<String> = vec!["udp-server".to_string()];
        let filename = "./tests/users-ref.kv".to_string();
        let handler = create_handler(Some(filename));

        assert!(handler.dbsize() >= 10);
        assert!(handler.status().contains("start_time"))
    }
}
