use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::fmt::Write;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpStream;

// pub type HttpFn = fn(HttpRequest) -> HttpResponse;
// #![feature(type_alias_impl_trait)]
pub type HttpFn = fn(HttpRequest) -> Vec<u8>;

#[derive(Debug, Default, Clone)]
pub struct HttpHeaders {
    headers: HashMap<String, String>,
}

pub trait Responder {
    fn transform(&self) -> Vec<u8>;
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

// pub struct Route {
//     'static
// }

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

#[derive(Debug, Default, Clone)]
pub enum HttpVersion {
    HTTP1_0,
    #[default]
    HTTP1_1,

}

#[derive(Debug, Default, Clone)]
pub enum StatusCode {
    #[default]
    Code200,
    Code404,
}

impl HttpVersion {
    pub fn to_string(&self) -> String {
        match self {
            HttpVersion::HTTP1_0 => "HTTP/1.0",
            HttpVersion::HTTP1_1 => "HTTP/1.1",
        }.to_string()
    }

    pub fn from_string(s: &String) -> Self {
        match s.as_str() {
            "HTTP/1.0" => Self::HTTP1_0,
            "HTTP/1.1" => Self::HTTP1_1,
            _ => unimplemented!(),
        }
    }
}

impl StatusCode {
    pub fn to_string(&self) -> String {
        match self {
            StatusCode::Code200 => "200 OK",
            StatusCode::Code404 => "404 Not Found",
        }.to_string()
    }
}

impl RequestType {
    pub fn from_string(s: &String) -> Self {
        match s.as_str() {
            "GET" => Self::GET,
            "HEAD" => Self::HEAD,
            "POST" => Self::POST,
            "PUT" => Self::PUT,
            "DELETE" => Self::DELETE,
            "CONNECT" => Self::CONNECT,
            "OPTIONS" => Self::OPTIONS,
            "TRACE" => Self::TRACE,
            "PATCH" => Self::PATCH,
            _ => unimplemented!()
        }
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    pub start_line: StartLine,
    pub headers: HttpHeaders,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status_line: StatusLine,
    pub headers: HttpHeaders,
    pub body: String,
}

#[derive(Debug)]
pub struct StartLine {
    pub request_type: RequestType,
    pub request_target: RequestTarget,
    pub http_version: HttpVersion,
}

#[derive(Debug, Default, Clone)]
pub struct StatusLine {
    pub protocol: HttpVersion,
    pub status_code: StatusCode,

}

#[derive(Debug)]
pub struct RequestTarget {
    pub uri: String,
    pub request_params: HashMap<String, String>,
}



impl HttpRequest {
    pub fn new(stream: &mut TcpStream) -> Result<Self, String> {

        let mut reader = BufReader::new(stream);

        let start_line = StartLine::new(&mut reader)?;
        let headers = Self::parse_headers(&mut reader)?;
        let content_length = headers.get("Content-Length").map_or("0", String::as_str);
        let content_length = content_length.parse::<usize>().unwrap();
        let body = Self::parse_body(&mut reader, content_length);

        Ok(Self {
            start_line,
            headers,
            body: body.unwrap(),
        })
    }

    fn parse_headers(reader: &mut BufReader<&mut TcpStream>) -> Result<HttpHeaders, String> {
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
                _ => return Err(format!("Could not parse header: {line}")),
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

    fn parse_body(reader: &mut BufReader<&mut TcpStream>, content_length: usize) -> Result<String, String> {
        let mut body = vec![];
        // let mut reader = BufReader::new(stream);
        let mut remaining = content_length;        // lines.push(self.body.clone());


        while remaining > 0 {
            let buf_size = std::cmp::min(remaining, 4096);
            let mut buf = vec![0u8; buf_size as usize];
            let len = reader.read(&mut buf).expect("test");
            if len == 0 {
                break;
            }
            body.extend_from_slice(&buf[..len]);
            remaining -= len as usize;
        }
        Ok(String::from_utf8(body).expect("Could not transform body to utf8"))
    }
}

impl StartLine {
    fn new(reader: &mut BufReader<&mut TcpStream>) -> Result<Self, String> {
        let mut line = String::new();
        let len = reader.read_line(&mut line).expect("Could not read start line");
        line = line.trim().to_string();

        let parts: Vec<String> = line.split(" ").map(str::to_string).collect();
        match &parts[..] {
            [request_type, request_target, http_version] => Ok(Self {
                request_type: RequestType::from_string(request_type),
                request_target: RequestTarget::new(request_target),
                http_version: HttpVersion::from_string(http_version),
            }),
            _ => Err(format!("Could not parse start line: {line}, {len}")),
        }
    }
}


impl RequestTarget {
    fn new(s: &String) -> Self {
        let parts: Vec<String> = s.split("?").map(str::to_string).collect();
        match &parts[..] {
            [uri] => Self  {
                uri: uri.to_owned(),
                request_params: HashMap::default(),
            },
            [uri, params] => {
                let mut request_params = HashMap::new();
                for param in params.split("&") {
                    let pair: Vec<String> = param.split("=").map(str::to_string).collect();
                    match &pair[..] {
                        [key, value] => request_params.insert(key.to_owned(), value.to_owned()),
                        _ => unimplemented!(),
                    };
                }

                Self {
                    uri: uri.to_owned(),
                    request_params,
                }
            },
            _ => unimplemented!()
        }
    }
}

impl HttpResponse {

    pub fn from_body(body: String) -> Self {
        let mut headers = HttpHeaders::default();
        headers.insert(String::from("Content-Length"), body.len().to_string());
        headers.insert(String::from("Connection"), String::from("close"));
        Self {
            status_line: Default::default(),
            headers: headers,
            body,
        }
    }
/* 
    pub fn send(&mut self, stream: &mut TcpStream) {
        let mut buf_writer = BufWriter::new(stream);
        let status_line = self.status_line.to_string();
        let length = self.body.len();
        self.headers.insert("Content-Length".to_string(), length.to_string());
        let mut lines = vec![status_line];

        for (k, v) in self.headers.iter() {
            lines.push(format!("{k}: {v}"));
        }

        let response = lines.join("\r\n");

        buf_writer.write_all(response.as_bytes()).unwrap();
        buf_writer.write_all("\r\n\r\n".as_bytes()).unwrap();
        buf_writer.write_all(self.body.as_bytes()).unwrap();
    }
    */
}
/* 
impl StatusLine {
    fn to_string(&self) -> String {
        format!("{0} {1}", self.protocol.to_string(), self.status_code.to_string())
    }
}
*/

// To Vec

impl From<HttpResponse> for Vec<u8> {
    fn from(http_response: HttpResponse) -> Self {
        let mut status_line: Vec<u8> = http_response.status_line.into();
        let mut headers: Vec<u8> = http_response.headers.into();
        let body = http_response.body.as_bytes();

        let size = status_line.len() + headers.len() + 4 + body.len();
        let mut buf = Vec::with_capacity(size);

        buf.append(&mut status_line);
        buf.append(&mut headers);
        buf.extend(b"\r\n");
        buf.extend_from_slice(body);
        buf
    }
}

impl From<StatusLine> for Vec<u8> {
    fn from(status_line: StatusLine) -> Self {
        let mut protocol: Vec<u8> = status_line.protocol.into();
        let mut status_code: Vec<u8> = status_line.status_code.into();
        let size = protocol.len() + status_code.len() + 3;
        let mut buf = Vec::with_capacity(size);

        buf.append(&mut protocol);
        buf.push(b' ');
        buf.append(&mut status_code);
        buf.extend(b"\r\n");
        buf
    }
}

impl From<HttpVersion> for Vec<u8> {
    fn from(http_version: HttpVersion) -> Self {
        http_version.to_string().into_bytes()
    }
}

impl From<StatusCode> for Vec<u8> {
    fn from(status_code: StatusCode) -> Self {
        status_code.to_string().into_bytes()
    }
}

impl From<HttpHeaders> for Vec<u8> {
    fn from(headers: HttpHeaders) -> Self {
        let mut buf = String::new();
        headers.iter()
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

impl Responder for Vec<u8> {
    fn transform(&self) -> Vec<u8> {
        self.to_owned()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_empty() {
        let response = HttpResponse::from_body(String::from(""));
        let expected = "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n".as_bytes().to_vec();
        let actual: Vec<u8> = response.into();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_into() {
        let response = HttpResponse::from_body("test".to_string());
        let expected = "HTTP/1.1 200 OK\r\nContent-Length: 4\r\n\r\ntest".as_bytes().to_vec();
        let actual: Vec<u8> = response.into();
        assert_eq!(expected, actual);
    }
}

