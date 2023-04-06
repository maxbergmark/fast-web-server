use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;
use std::slice::Iter;

pub type HttpFn = fn(HttpRequest) -> HttpResponse;
pub type HttpHeaders = HashMap<String, String>;

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

#[derive(Debug, Default)]
pub enum HttpVersion {
    HTTP1_0,
    #[default]
    HTTP1_1,

}

#[derive(Debug, Default)]
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

#[derive(Debug)]
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

#[derive(Debug, Default)]
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

        let lines = Self::get_lines(stream);
        // println!("{:?}", lines);
        let mut line_iter = lines.iter();

        let start_line = StartLine::new(&mut line_iter)?;
        let headers = Self::parse_headers(&mut line_iter)?;
        let body = Self::parse_body(&mut line_iter);

        Ok(Self {
            start_line,
            headers,
            body,
        })
    }

    fn get_lines(stream: &mut TcpStream) -> Vec<String> {
        BufReader::new(stream)
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect()
    }

    fn parse_headers(line_iter: &mut Iter<String>) -> Result<HttpHeaders, String> {
        let mut headers = HttpHeaders::new();
        for line in line_iter.by_ref() {
            if line.is_empty() {
                break;
            }
            let parts: Vec<&str> = line.splitn(2, ":").collect();
            let (key, value) = match parts[..] {
                [a, b] => (a, b),
                _ => return Err(format!("Could not parse header: {line}")),
            };
            headers.insert(key.to_owned(), value.to_owned());
        }
        Ok(headers)
    }

    fn parse_body(line_iter: &mut Iter<String>) -> String {
        match line_iter.next() {
            Some(s) => s,
            None => ""
        }.to_string()
    }
}

impl StartLine {
    fn new(line_iter: &mut Iter<String>) -> Result<Self, String> {
        let s = match line_iter.by_ref().next() {
            Some(s) =>  s,
            None => return Err("No start line".to_string()),
        };
        let parts: Vec<String> = s.split(" ").map(str::to_string).collect();
        match &parts[..] {
            [request_type, request_target, http_version] => Ok(Self {
                request_type: RequestType::from_string(request_type),
                request_target: RequestTarget::new(request_target),
                http_version: HttpVersion::from_string(http_version),
            }),
            _ => Err(format!("Could not parse start line: {s}")),
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
        Self {
            status_line: Default::default(),
            headers: Default::default(),
            body,
        }
    }

    pub fn send(&self, stream: &mut TcpStream) {
        let mut buf_writer = BufWriter::new(stream);
        let status_line = self.status_line.to_string();
        let length = self.body.len();

        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{0}", self.body);

        buf_writer.write_all(response.as_bytes()).unwrap();
    }
}

impl StatusLine {
    fn to_string(&self) -> String {
        format!("{0} {1}", self.protocol.to_string(), self.status_code.to_string())
    }
}
