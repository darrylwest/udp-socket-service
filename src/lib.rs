pub mod config;
/// the modules
pub mod server;

/// the current app version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// the required api key clients use in all reqests
pub const APIKEY: (&str, &str) = ("apikey", "6bb31c92d5e441fba1a24e81770409e4");

/// the name of the pid-file created when the service starts
pub const SERVER_PID_FILE: &str = "udp-service.pid";
