use std::collections::{HashMap, hash_map::Iter};
use std::fmt::Write;

use itertools::Itertools;


#[derive(Debug, Default, Clone)]
pub struct HttpHeaders {
    headers: HashMap<String, String>,
}

impl HttpHeaders {
    pub fn new() -> Self {
        Self {
            headers: HashMap::new(),
        }
    }

    pub fn iter(&self) -> Iter<String, String> {
        self.headers.iter()
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }
}

impl Into<Vec<u8>> for HttpHeaders {
    fn into(self) -> Vec<u8> {
        let mut buf = String::new();
        self.headers.iter().sorted()
            .fold(
                &mut buf, 
                |buf, (k, v)| {
                    write!(buf, "{}: {}\r\n", k, v).unwrap();
                    buf
                }
        );
        buf.into_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let headers = HttpHeaders::new();
        assert!(headers.headers.is_empty());
    }

    #[test]
    fn test_insert() {
        let mut headers = HttpHeaders::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        assert_eq!(headers.get("Content-Type"), Some(&"application/json".to_string()));
    }

    #[test]
    fn test_get() {
        let mut headers = HttpHeaders::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        assert_eq!(headers.get("Content-Type"), Some(&"application/json".to_string()));
        assert_eq!(headers.get("Accept"), None);
    }

    #[test]
    fn test_iter() {
        let mut headers = HttpHeaders::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Accept".to_string(), "text/html".to_string());
        let iter = headers.iter();
        assert_eq!(iter.count(), 2);
    }

    #[test]
    fn test_into() {
        let mut headers = HttpHeaders::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Accept".to_string(), "text/html".to_string());
        let expected_bytes = b"Accept: text/html\r\nContent-Type: application/json\r\n".to_vec();
        let bytes: Vec<u8> = headers.into();
        assert_eq!(bytes, expected_bytes);
    }
}