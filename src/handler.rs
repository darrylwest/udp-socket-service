use crate::parsers::split2;
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
        let (cmd, params) = split2(msg);
        match cmd.as_str() {
            "" => Err(anyhow!("empty request")),
            _ => {
                let (key, value) = split2(&params);
                let params = if value.is_empty() {
                    vec![key]
                } else {
                    vec![key, value]
                };

                Ok(Request { cmd, params })
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

    /// return the body as a usize int
    pub fn as_number<T: std::str::FromStr>(&self) -> Result<T>
    where
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let value = self.body.as_str();
        match value.parse::<T>() {
            Ok(n) => Ok(n),
            Err(e) => {
                let msg = format!("parse: {} error: {:?}", value, e);
                error!("{}", msg);
                Err(anyhow!("{}", msg))
            }
        }
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
    pub fn handle_request(&mut self, request: Request) -> Response {
        info!("handle request: {}", &request.cmd);
        match request.cmd.as_str() {
            "ping" => Response::create_ok("PONG".to_string()),
            "now" => Response::create_ok(format!("{}", get_ts())),
            "now_ns" => Response::create_ok(format!("{}", get_ns())),
            "get" => {
                println!("get {:?}", &request.params);
                let key = request.params[0].as_str();
                self.get(key)
            }
            "set" => {
                println!("set {:?}", &request.params);
                if request.params.len() == 2 {
                    let key = request.params[0].as_str();
                    let value = request.params[1].as_str();
                    self.set(key, value)
                } else {
                    Response::create(Status::bad_request(), request.cmd.to_string())
                }
            }
            "del" => {
                println!("del {:?}", &request.params);
                let key = request.params[0].as_str();
                self.del(key)
            }
            "dbsize" => {
                let sz = self.db.dbsize();
                Response::create_ok(sz.to_string())
            }
            _ => {
                error!("bad request: {}", &request.cmd);
                Response::create(Status::bad_request(), request.cmd.to_string())
            }
        }
    }

    /// get the item from key
    fn get(&self, key: &str) -> Response {
        match self.db.get(key) {
            Some(value) => Response::create_ok(value),
            _ => Response::create(Status::not_found(), key.to_string()),
        }
    }

    /// set the value from key
    fn set(&mut self, key: &str, value: &str) -> Response {
        let _ = self.db.set(key, value);
        Response::create_ok(value.to_string())
    }

    fn del(&mut self, key: &str) -> Response {
        if let Some(resp) = self.db.remove(key) {
            Response::create_ok(resp)
        } else {
            Response::create_ok("ok".to_string())
        }
    }
}

fn get_ts() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        _ => 0_u64,
    }
}

fn get_ns() -> u128 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => n.as_nanos(),
        _ => 0_u128,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_handler() -> Handler {
        let db = DataStore::create();
        Handler::new(db)
    }

    #[test]
    fn get_set_del_dbsize() {
        let mut handler = create_handler();
        assert_eq!(handler.db.dbsize(), 0);

        let key = "1234.MyKey";
        let value = "This is a test value";
        let msg = format!("set {} {}", key, value);
        let request = Request::from_message(msg.as_str()).unwrap();
        let response = handler.handle_request(request);
        println!("{:?}", response);
        assert_eq!(handler.db.dbsize(), 1);

        let msg = format!("get {}", key);
        let request = Request::from_message(msg.as_str()).unwrap();
        let response = handler.handle_request(request);
        println!("{:?}", response);
        let request = Request::from_message("dbsize").unwrap();
        let response = handler.handle_request(request);
        println!("dbsize {:?}", response);
        assert_eq!(handler.db.dbsize(), 1);
        assert_eq!(response.as_number::<usize>().unwrap(), 1);
    }

    #[test]
    fn new() {
        let db = DataStore::create();
        let handler = Handler::new(db);

        assert_eq!(handler.db.dbsize(), 0);
    }
}
