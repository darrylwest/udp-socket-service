///
use anyhow::{anyhow, Result};
use log::{error, info};
use std::time::{SystemTime, UNIX_EPOCH};
use tiny_kv::db::DataStore;

#[derive(Debug, Default, Clone)]
pub struct Request {
    pub cmd: String,
    pub params: Vec<String>,
}

impl Request {
    /// parse the incoming message and return a request object or none
    pub fn from_message(msg: &str) -> Result<Request> {
        let params: Vec<&str> = msg.split(' ').collect();
        match params.len() {
            0 => Err(anyhow!("empty request")),
            _ => {
                let cmd = params[0].to_string();
                let mut p: Vec<String> = Vec::new();
                for param in params {
                    p.push(param.to_string());
                }

                Ok(Request { cmd, params: p })
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Status {
    pub code: u16,
    pub description: String,
}

impl Status {
    pub fn ok() -> Status {
        let code: u16 = 200;
        Status {
            code,
            description: "ok".to_string(),
        }
    }

    pub fn bad_request() -> Status {
        let code: u16 = 400;
        Status {
            code,
            description: "bad-request".to_string(),
        }
    }

    pub fn not_found() -> Status {
        let code: u16 = 404;
        Status {
            code,
            description: "not-found".to_string(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Response {
    pub status: Status,
    pub body: String,
}

impl Response {
    /// create an ok response
    pub fn create_ok(body: String) -> Response {
        let status = Status::ok();
        Response { status, body }
    }

    /// used to return status that is other than ok/200; usually an error
    pub fn create(status: Status, body: String) -> Response {
        Response { status, body }
    }

    /// return the formatted response as a string
    pub fn as_string(&self) -> String {
        format!(
            "{}:{}:{}",
            self.status.code, self.status.description, self.body
        )
    }
}

#[derive(Clone)]
pub struct Handler {
    pub db: DataStore,
}

impl Handler {
    /// create a new handler with the specified data store.
    pub fn new(db: DataStore) -> Handler {
        Handler { db }
    }

    /// returns a response to the request, including error responses
    pub fn handle_request(&self, request: Request) -> Response {
        info!("handle request: {}", &request.cmd);
        match request.cmd.as_str() {
            "ping" => Response::create_ok("PONG".to_string()),
            "now" => Response::create_ok(format!("{}", get_now())),
            "get" => {
                if request.params.len() == 1 {
                    let key = request.params[0].as_str();
                    self.get(key)
                } else {
                    Response::create(Status::bad_request(), request.cmd.to_string())
                }
            }
            _ => {
                error!("bad request: {}", &request.cmd);
                Response::create(Status::bad_request(), request.cmd.to_string())
            }
        }
    }

    /// get the item
    fn get(&self, key: &str) -> Response {
        match self.db.get(key) {
            Some(value) => Response::create_ok(value),
            _ => Response::create(Status::not_found(), key.to_string()),
        }
    }
}

fn get_now() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        _ => 0_u64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let db = DataStore::create();
        let handler = Handler::new(db);
        let _h = handler.clone();

        assert!(true);
    }
}
