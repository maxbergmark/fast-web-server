use std::collections::HashMap;
// use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::{TcpListener, TcpStream};
// use std::slice::Iter;
use rayon::ThreadPool;
// use fast_web_server_macros::fast_web_server_routes;
use fast_web_server_types::{HttpFn, HttpRequest, HttpResponse, HttpVersion, RequestType, StatusCode, StatusLine};


pub struct FastWebServer {
    listener: TcpListener,
    thread_pool: ThreadPool,
    routes: HashMap<(RequestType, String), HttpFn>,
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
            routes: HashMap::default()
        }
    }

    pub fn bind(&mut self, request_type: RequestType, route: &str, func: HttpFn) {
        self.routes.insert((request_type, route.to_string()), func);
    }

    pub fn run(&self) -> Result<(), String> {
        for stream in self.listener.incoming() {
            println!("new client");
            self.handle_connection(stream.unwrap());
        }
        Ok(())
    }

    // #[fast_web_server_routes]
    // fn add_routes(&self) {
    //
    // }

    fn handle_connection(&self, stream: TcpStream) {
        // self.thread_pool.execute(||  {
        let routes = self.routes.clone();
        self.thread_pool.spawn(||  {
            Self::handle_client(routes, stream);
        });
    }



    fn handle_client(routes: HashMap<(RequestType, String), HttpFn>, mut stream: TcpStream) {

        let http_request = match HttpRequest::new(&mut stream) {
            Ok(request) => request,
            Err(_message) => return,
        };

        let request_type = http_request.start_line.request_type.to_owned();
        let path = http_request.start_line.request_target.uri.to_owned();
        let key = (request_type, path);
        // let body = format!("<html><body><h1>{path}</h1></body></html>");

        let response = match routes.get(&key) {
            Some(func) => func(http_request),
            None => Self::get_404(),
        };

        response.send(&mut stream);
    }

    fn get_404() -> HttpResponse {
        HttpResponse {
            status_line: StatusLine {
                protocol: HttpVersion::HTTP1_1,
                status_code: StatusCode::Code404
            },
            headers: Default::default(),
            body: "".to_string(),
        }
    }
}

