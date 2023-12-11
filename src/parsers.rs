/// standard parsers used by handlers and clients
use anyhow::{anyhow, Result};
use log::error;

/// split the message into head and tail using whitespace as delim
pub fn split2(msg: &str) -> (String, String) {
    let mut split = msg.split_whitespace();
    let head = split.next().unwrap_or("");
    let mut tail = String::new();
    for s in split {
        tail.push_str(s);
        tail.push(' ');
    }

    (head.to_string(), tail.trim().to_string())
}

/// return the body as a usize int
pub fn as_number<T: std::str::FromStr>(value: &str) -> Result<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    match value.parse::<T>() {
        Ok(n) => Ok(n),
        Err(e) => {
            let msg = format!("parse: {} error: {:?}", value, e);
            error!("{}", msg);
            Err(anyhow!("{}", msg))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::split2;

    #[test]
    fn test_split2_kv() {
        let msg = "set key my wordy key";
        let (left, right) = split2(msg);
        println!("{}: {}", left, right);
        assert_eq!(left, "set");
        assert_eq!(right, "key my wordy key");
    }

    #[test]
    fn test_split2_k() {
        let msg = "get key";
        let (left, right) = split2(msg);
        println!("{}: {}", left, right);
        assert_eq!(left, "get");
        assert_eq!(right, "key");
    }
}
