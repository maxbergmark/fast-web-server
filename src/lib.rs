use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpStream;

type HttpHeaders = HashMap<String, String>;

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

#[derive(Debug)]
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

impl HttpRequest {
    pub fn new(stream: &mut TcpStream) -> Result<Self, &str> {
        let buf_reader = BufReader::new(stream);
        let lines: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        // println!("{:?}", lines);
        
        let start_line = match lines.first() {
            Some(s) => StartLine::new(s),
            None => return Err("No header"),
        };

        let mut headers = HttpHeaders::new();
        let mut line_iter = lines.iter().skip(1);
        for line in line_iter.by_ref() {
            if line.is_empty() {
                break;
            }
            let parts: Vec<String> = line.split(":").map(str::to_string).collect();
            let (key, value) = match parts[..] {
                (a, b);
            };
            let key = parts.get(0).unwrap().to_owned();
            let value = parts.get(1).unwrap().to_owned();
            headers.insert(key, value);
        }
        let body = match line_iter.next() {
            Some(s) => s,
            None => ""
        };

        Ok(Self {
            start_line,
            headers: Default::default(),
            body: body.to_string(),
        })
    }
}

impl StartLine {
    fn new(s: &String) -> Self {
        let parts: Vec<String> = s.split(" ").map(str::to_string).collect();
        match &parts[..] {
            [a, b, c] => Self {
                request_type: RequestType::from_string(a),
                request_target: RequestTarget::new(b),
                http_version: HttpVersion::from_string(c),
            },
            _ => unimplemented!()
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
    
    pub fn from_body(body: &str) -> Self {
        Self {
            status_line: Default::default(),
            headers: Default::default(),
            body: body.to_string(),
        }
    }

    pub fn send(&self, stream: &mut TcpStream) {
        let mut buf_writer = BufWriter::new(stream);
        let status_line = self.status_line.to_string();
        let length = self.body.len();

        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{0}", self.body);

        // println!("{}", response);
        buf_writer.write_all(response.as_bytes()).unwrap();
        // stream.write_all(response.as_bytes()).unwrap();
    }
}

impl StatusLine {
    fn to_string(&self) -> String {
        format!("{0} {1}", self.protocol.to_string(), self.status_code.to_string())
    }
}

impl HttpVersion {
    fn to_string(&self) -> String {
        match self {
            HttpVersion::HTTP1_0 => "HTTP/1.0",
            HttpVersion::HTTP1_1 => "HTTP/1.1",
        }.to_string()
    }

    fn from_string(s: &String) -> Self {
        match s.as_str() {
            "HTTP/1.0" => Self::HTTP1_0,
            "HTTP/1.1" => Self::HTTP1_1,
            _ => unimplemented!(),
        }
    }
}

impl StatusCode {
    fn to_string(&self) -> String {
        match self {
            StatusCode::Code200 => "200 OK",
            StatusCode::Code404 => "404 Not Found",
        }.to_string()
    }
}

impl RequestType {
    fn from_string(s: &String) -> Self {
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