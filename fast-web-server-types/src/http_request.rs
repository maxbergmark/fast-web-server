use std::{io::{BufReader, Read, BufRead, self, ErrorKind}, error::Error};

use thiserror::Error;

use crate::{start_line::StartLine, HttpHeaders};


#[derive(Debug)]
pub struct HttpRequest {
    pub start_line: StartLine,
    pub headers: HttpHeaders,
    pub body: String,
}

impl HttpRequest {
    pub fn new(stream: &mut dyn Read) -> Result<Self, Box<dyn Error>> {

        let mut reader: BufReader<&mut dyn Read> = BufReader::new(stream);

        let start_line = StartLine::new(&mut reader)?;
        let headers = Self::parse_headers(&mut reader)?;
        let content_length = headers.get("Content-Length").map_or("0", String::as_str);
        let content_length = content_length.parse::<usize>().unwrap();
        let body = Self::parse_body(&mut reader, content_length);

        Ok(Self {
            start_line,
            headers,
            body: body?,
        })
    }

    fn parse_headers(reader: &mut dyn BufRead) -> Result<HttpHeaders, Box<dyn Error>> {
        let mut headers = HttpHeaders::new();
        loop {
            let mut line = String::new();
            let len = reader.read_line(&mut line).expect("Could not read header line");
            line = line.trim().to_string();
            if len == 0 || line.is_empty() {
                break;
            }
            let parts: Vec<&str> = line.splitn(2, ": ").collect();
            let (key, value) = match parts[..] {
                [a, b] => (a, b),
                _ => return Err(HttpRequestError(String::from("Could not parse header")).into()),
            };
            headers.insert(key.to_owned(), value.to_owned());
        }
        // reader.take(2).read_to_string(&mut line);
        // match reader.read_line(&mut line) {
            // Ok(_) => {},
            // Err(e) => return Err(e.to_string()),
        // }
        Ok(headers)
    }

    fn parse_body(reader: &mut dyn BufRead, content_length: usize) -> Result<String, io::Error> {
        let mut body = vec![];
        let mut remaining = content_length;
        let mut buf = [0u8; 4096];

        while remaining > 0 {
            let len = reader.read(&mut buf)?;
            if len == 0 {
                return Err(io::Error::new(ErrorKind::InvalidData, "Could not read entire body"));
            }
            body.extend_from_slice(&buf[..len]);
            remaining -= len as usize;
        }
        Ok(String::from_utf8(body).expect("Could not transform body to utf8"))
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("{0}")]
struct HttpRequestError(String);

#[cfg(test)]
mod tests {
    use std::{io::{Cursor, ErrorKind}, collections::HashMap};

    use crate::{http_request::{HttpRequest}, start_line::StartLine, RequestType, request_target::RequestTarget};

    #[test]
    fn test_parse_headers() {
        let mut input = b"Content-Type: text/plain\r\nUser-Agent: curl/7.64.1\r\n\r\n" as &[u8];
        let headers = HttpRequest::parse_headers(&mut input).unwrap();
        assert_eq!(headers.get("Content-Type"), Some(&"text/plain".to_owned()));
        assert_eq!(headers.get("User-Agent"), Some(&"curl/7.64.1".to_owned()));
        assert_eq!(headers.get("Content-Length"), None);
    }
    #[test]
    fn test_parse_headers_with_invalid_header() {
        let mut input = b"Content-Type text/plain\r\n\r\n" as &[u8];
        let result = HttpRequest::parse_headers(&mut input);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().to_string(), "Could not parse header");
    }

    #[test]
    fn test_parse_body() {
        let mut input = b"hello world" as &[u8];
        let len = input.len();
        let body = HttpRequest::parse_body(&mut input, len).unwrap();
        assert_eq!(body, "hello world");
    }

    #[test]
    fn test_parse_body_with_incomplete_input() {
        let mut input = b"hello world" as &[u8];
        let len = input.len();
        let body = HttpRequest::parse_body(&mut input, len + 1); // input.len() + 1 bytes
        assert!(body.is_err());
        let err = body.err().unwrap().kind();
        assert_eq!(err, ErrorKind::InvalidData);
    }

    #[test]
    fn test_new() {
        let input = b"GET / HTTP/1.1\r\nHost: localhost:3000\r\nContent-Type: text/plain\r\nContent-Length: 11\r\n\r\nhello world";
        let mut stream = Cursor::new(input.as_ref());
        let request = HttpRequest::new(&mut stream).unwrap();
        let expected = StartLine { request_type: RequestType::GET, request_target: RequestTarget {uri: String::from("/"), request_params: HashMap::default() }, http_version: crate::HttpVersion::HTTP1_1 };
        assert_eq!(request.start_line, expected);
        let headers = request.headers;
        assert_eq!(headers.get("Host"), Some(&"localhost:3000".to_owned()));
        assert_eq!(headers.get("Content-Type"), Some(&"text/plain".to_owned()));
        assert_eq!(headers.get("Content-Length"), Some(&"11".to_owned()));
        assert_eq!(request.body, "hello world");
    }

    #[test]
    fn test_new_with_incomplete_input() {
        let input = b"GET / HTTP/1.1\r\nHost: localhost:3000\r\nContent-Type: text/plain\r\nContent-Length: 12\r\n\r\nhello world";
        let mut stream = Cursor::new(&input[..(input.len() - 1)]); // last byte missing
        let result = HttpRequest::new(&mut stream);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().to_string(), "Could not read entire body");
    }
}