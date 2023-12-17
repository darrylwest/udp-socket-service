///
use crate::parsers;
use anyhow::{anyhow, Result};
use log::{error, info};
use service_uptime::status::ServiceStatus;
use std::time::{SystemTime, UNIX_EPOCH};
use tiny_kv::db::DataStore;

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct Request {
    pub cmd: String,
    pub params: Vec<String>,
}

impl Request {
    /// parse the incoming message and return a request object or none
    pub fn from_message(msg: &str) -> Result<Request> {
        let (cmd, params) = parsers::split2(msg);
        match cmd.as_str() {
            "" => Err(anyhow!("empty request")),
            _ => {
                let (key, value) = parsers::split2(&params);
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

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
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

#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
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

    /// parse the body into a usize int
    pub fn as_usize(&self) -> Result<usize> {
        parsers::as_number::<usize>(self.body.as_str())
    }

    /// parse the body and convert to u64
    pub fn as_u64(&self) -> Result<u64> {
        parsers::as_number::<u64>(self.body.as_str())
    }

    /// return the formatted response as a string
    pub fn as_string(&self) -> String {
        format!(
            "{}:{}:{}",
            self.status.code, self.status.description, self.body
        )
    }
}

#[derive(Debug, Default, Clone)]
pub struct Handler {
    db: DataStore,
    status: ServiceStatus,
}

impl Handler {
    /// create a new handler with the specified data store.
    pub fn new(db: DataStore) -> Handler {
        Handler {
            db,
            status: ServiceStatus::create(),
        }
    }

    /// returns a response to the request, including error responses
    pub fn handle_request(&mut self, request: Request) -> Response {
        self.status.access.incr();
        info!("handle request: {}", &request.cmd);

        match request.cmd.as_str() {
            "ping" => Response::create_ok("PONG".to_string()),
            "now" => Response::create_ok(format!("{}", get_ts())),
            "now_ns" => Response::create_ok(format!("{}", get_ns())),
            "status" => Response::create_ok(format!("{}", self.status)),
            "get" => {
                info!("get {:?}", &request.params);
                let key = request.params[0].as_str();
                self.get(key)
            }
            "set" => {
                info!("set {:?}", &request.params);
                if request.params.len() == 2 {
                    let key = request.params[0].as_str();
                    let value = request.params[1].clone().into_bytes();
                    self.set(key, value)
                } else {
                    Response::create(Status::bad_request(), request.cmd.to_string())
                }
            }
            "del" => {
                info!("del {:?}", &request.params);
                let key = request.params[0].as_str();
                self.del(key)
            }
            "dbsize" => {
                let sz = self.dbsize();
                Response::create_ok(sz.to_string())
            }
            "keys" => {
                let keys = self.db.keys();
                let body = format!("{:?}", keys);
                Response::create_ok(body)
            }
            "loaddb" => {
                let filename = request.params[0].as_str();
                if let Ok(sz) = self.db.loaddb(filename) {
                    Response::create_ok(sz.to_string())
                } else {
                    Response::create(Status::bad_request(), filename.to_string())
                }
            }
            "savedb" => {
                let filename = request.params[0].as_str();
                if let Ok(sz) = self.db.savedb(filename) {
                    Response::create_ok(sz.to_string())
                } else {
                    Response::create(Status::bad_request(), filename.to_string())
                }
            }
            _ => {
                self.status.errors.incr();
                error!("bad request: {}", &request.cmd);
                Response::create(Status::bad_request(), request.cmd.to_string())
            }
        }
    }

    /// get the item from key
    fn get(&self, key: &str) -> Response {
        match self.db.get(key) {
            Some(value) => {
                let body = String::from_utf8(value).unwrap();
                Response::create_ok(body)
            }
            _ => Response::create(Status::not_found(), key.to_string()),
        }
    }

    /// set the value from key
    fn set(&mut self, key: &str, value: Vec<u8>) -> Response {
        if let Some(value) = self.db.set(key, value) {
            let val = String::from_utf8(value).unwrap();
            Response::create_ok(val)
        } else {
            Response::create_ok("ok".to_string())
        }
    }

    fn del(&mut self, key: &str) -> Response {
        if let Some(value) = self.db.remove(key) {
            let val = String::from_utf8(value).unwrap();
            Response::create_ok(val)
        } else {
            Response::create_ok("ok".to_string())
        }
    }

    /// return the number of elements in the k/v map
    pub fn dbsize(&self) -> usize {
        self.db.dbsize()
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
        info!("{:?}", response);
        assert_eq!(handler.db.dbsize(), 1);

        let msg = format!("get {}", key);
        let request = Request::from_message(msg.as_str()).unwrap();
        let response = handler.handle_request(request);
        info!("{:?}", response);
        let request = Request::from_message("dbsize").unwrap();
        let response = handler.handle_request(request);
        info!("dbsize {:?}", response);
        assert_eq!(handler.db.dbsize(), 1);
        assert_eq!(response.as_usize().unwrap(), 1);
    }

    #[test]
    fn bad_set() {
        let mut handler = create_handler();
        let request = Request {
            cmd: "set".to_string(),
            params: vec!["mykey".to_string()],
        };
        let response = handler.handle_request(request);
        assert_eq!(response.status.code, 400);
    }

    #[test]
    fn bad_get() {
        let mut handler = create_handler();
        let request = Request {
            cmd: "get".to_string(),
            params: vec!["my-bad-key".to_string()],
        };
        let response = handler.handle_request(request);
        assert_eq!(response.status.code, 404);
    }

    #[test]
    fn del() {
        let mut handler = create_handler();
        let rq = Request {
            cmd: "set".to_string(),
            params: vec!["mykey".to_string(), "my value".to_string()],
        };
        let _ = handler.handle_request(rq);

        assert_eq!(handler.db.dbsize(), 1);

        let request = Request {
            cmd: "del".to_string(),
            params: vec!["mykey".to_string()],
        };
        let response = handler.handle_request(request.clone());
        assert_eq!(response.status.code, 200);
        let response = handler.handle_request(request.clone());
        assert_eq!(response.status.code, 200);
    }

    #[test]
    fn new() {
        let db = DataStore::create();
        let handler = Handler::new(db);

        assert_eq!(handler.db.dbsize(), 0);
    }

    #[test]
    fn status_bad_request() {
        let status = Status::bad_request();
        assert_eq!(status.code, 400);
    }

    #[test]
    fn status_not_found() {
        let status = Status::not_found();
        assert_eq!(status.code, 404);
    }

    #[test]
    fn create_response() {
        let status = Status::not_found();
        let response = Response::create(status, "this is a test".to_string());
        assert_eq!(response.status.code, 404);
    }

    #[test]
    fn response_as_u64() {
        let response = Response::create_ok("42".to_string());
        let n = response.as_u64();
        assert!(n.is_ok());
        assert_eq!(n.unwrap(), 42);
    }

    #[test]
    fn response_as_string() {
        let response = Response::create_ok("this is the body part".to_string());
        let ss = response.as_string();
        assert_eq!(ss, "200:ok:this is the body part");
    }
}
