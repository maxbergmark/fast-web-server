use std::{error::Error, fmt};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum RequestType {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl RequestType {
    pub fn from_string(s: &String) -> Result<Self, RequestTypeError> {
        let request_type = match s.as_str() {
            "GET" => Self::GET,
            "HEAD" => Self::HEAD,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "DELETE" => Self::DELETE,
            "CONNECT" => Self::CONNECT,
            "OPTIONS" => Self::OPTIONS,
            "TRACE" => Self::TRACE,
            "PATCH" => Self::PATCH,
            _ => return Err(RequestTypeError {})
        };
        Ok(request_type)
    }

    pub fn from_str(s: &str) -> Result<Self, RequestTypeError> {
        Self::from_string(&String::from(s))
    }
}

#[derive(Debug)]
pub struct RequestTypeError;

impl Error for RequestTypeError {}

impl fmt::Display for RequestTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse request type")
    }
}

#[cfg(test)]
mod tests {
    use crate::{RequestType, request_type::RequestTypeError};

    #[test]
    fn simple_get() {
        let actual = RequestType::from_str("GET");
        assert_eq!(RequestType::GET, actual.unwrap())
    }


    #[test]
    fn unimplemented() {
        let _actual = RequestType::from_str("unimplemented");
        assert!(matches!(RequestTypeError, _actual))
    }
}