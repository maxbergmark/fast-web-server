use std::collections::HashMap;
use std::io::{Write, BufWriter, ErrorKind};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use rayon::ThreadPool;
use fast_web_server_types::{HttpFn, HttpRequest, HttpResponse, HttpVersion, RequestType, StatusCode, StatusLine, HttpHeaders};


pub struct FastWebServer {
    listener: TcpListener,
    thread_pool: ThreadPool,
    routes: Arc<RwLock<HashMap<(RequestType, String), HttpFn>>>,
}

impl FastWebServer {
    pub fn new(addr: &str, num_workers: usize) -> Self {

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_workers)
            .build()
            .unwrap();

        Self {
            listener: TcpListener::bind(addr).unwrap(),
            // thread_pool: ThreadPool::new(num_workers),
            thread_pool: pool,
            routes: Arc::new(RwLock::new(HashMap::default())),
        }
    }

    pub fn bind(&mut self, request_type: RequestType, route: &str, func: HttpFn) {
        let mut routes = self.routes.write().unwrap();
        routes.insert((request_type, route.to_string()), func);
    }

    pub fn run(&self) -> Result<(), String> {
        for stream in self.listener.incoming() {
            self.handle_connection(stream.unwrap());
        }
        Ok(())
    }

    fn handle_connection(&self, 
        // routes: Arc<RwLock<HashMap<(RequestType, String), HttpFn>>>, 
        stream: TcpStream) {

            let routes = self.routes.clone();
            self.thread_pool.spawn(||  {
            match Self::handle_client(routes, stream) {
                Ok(_) => {},
                Err(e) => eprintln!("{}", e),
            }
        });
    }



    fn handle_client(
        routes: Arc<RwLock<HashMap<(RequestType, String), HttpFn>>>, 
        mut stream: TcpStream) -> std::io::Result<()> {

        let http_request = match HttpRequest::new(&mut stream) {
            Ok(request) => request,
            Err(e) => return Err(std::io::Error::new(ErrorKind::Other, e.to_string())),
        };

        let request_type = http_request.start_line.request_type.to_owned();
        let path = http_request.start_line.request_target.uri.to_owned();
        let key = (request_type, path);

        let response = match routes.read().unwrap().get(&key) {
            Some(func) => func(http_request),
            None => Self::get_404().into(),
        };
        // let response = Self::get_404().into();
        let http_response = HttpResponse::from_body(String::from_utf8(response).unwrap());
        let response_vec: Vec<u8> = http_response.into();

        let mut writer = BufWriter::new(stream);
        match writer.write_all(&response_vec) {
            Ok(_) => {},
            Err(e) => return Err(e),
        }
        writer.flush()
    }

    fn get_404() -> HttpResponse {
        let mut headers = HttpHeaders::default();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Connection".to_string(), "close".to_string());
        
        HttpResponse {
            status_line: StatusLine {
                protocol: HttpVersion::HTTP1_1,
                status_code: StatusCode::Code404
            },
            headers: headers,
            body: "{\"error\": \"not_found\"}".to_string(),
        }
    }
}
