use std::net::{TcpListener, TcpStream};
use rayon::ThreadPool;
use fast_web_server::{HttpRequest, HttpResponse, HttpVersion, StatusCode, StatusLine};
use std::io::Result;

struct FastWebServer {
    listener: TcpListener,
    thread_pool: ThreadPool,
}

impl FastWebServer {
    fn new(addr: &str, num_workers: usize) -> Self {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_workers)
            .build()
            .unwrap();
        Self {
            listener: TcpListener::bind(addr).unwrap(),
            // thread_pool: ThreadPool::new(num_workers),
            thread_pool: pool,
        }
    }

    fn run(&self) -> Result<()> {
        for stream in self.listener.incoming() {
            self.handle_connection(stream.unwrap());
        }
        Ok(())
    }

    fn handle_connection(&self, stream: TcpStream) {
        // self.thread_pool.execute(||  {
        self.thread_pool.spawn(||  {
            Self::handle_client(stream);
        });
    }



    fn handle_client(mut stream: TcpStream) {

        let http_request = match HttpRequest::new(&mut stream) {
            Ok(request) => request,
            Err(_message) => return,
        };

        // println!("{:?}", http_request);
        // thread::sleep(Duration::from_millis(10));

        let path = http_request.start_line.request_target.uri;

        // let mut s = 0;
        // for _i in 0..1000000 {
        //     s = s + rand::thread_rng().gen_range(0..100);
        // }

        let body = format!("<html><body><h1>{path}</h1></body></html>");

        let response = HttpResponse {
            status_line: StatusLine {
                protocol: HttpVersion::HTTP1_1,
                status_code: StatusCode::Code200
            },
            headers: Default::default(),
            body: body.to_string(),
        };

        response.send(&mut stream);
    }
}


fn main() -> Result<()> {
    let server = FastWebServer::new("0.0.0.0:7878", 40);
    server.run()
}



// #[get("/test")]
// fn test_getter(request: HttpRequest) -> HttpResponse {
//     HttpResponse::from_body("test")
// }


