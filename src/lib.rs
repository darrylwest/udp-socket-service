pub mod client;
///
/// the modules
///
pub mod config;
pub mod handler;
pub mod server;

/// the current app version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// the name of the pid-file created when the service starts
pub const SERVER_PID_FILE: &str = "udp-service.pid";
