use fast_web_server_impl::{FastWebServer};
use fast_web_server_types::{HttpRequest, HttpResponse, RequestType};

fn main() -> Result<(), String> {
    let mut server = FastWebServer::new("0.0.0.0:7878", 4);
    server.bind(RequestType::GET, "/test", test_getter);
    // server.bind(RequestType::GET, "/other-test", other_test_getter);
    // server.bind(RequestType::POST, "/post-mirror", mirror_response);
    server.run()
}


// #[get("/test")]
fn test_getter(_request: HttpRequest) -> HttpResponse {
    HttpResponse::from_body("this is test_getter".to_string())
}

fn other_test_getter(_request: HttpRequest) -> HttpResponse {
    HttpResponse::from_body("this is other_test_getter".to_string())
}

fn mirror_response(request: HttpRequest) -> HttpResponse {
    HttpResponse::from_body(request.body)
}
