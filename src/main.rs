use fast_web_server_impl::{FastWebServer, RegisterEndpoint, bind};
use fast_web_server_macros::{get, post};
use fast_web_server_types::{HttpRequest, RequestType};

fn main() -> Result<(), String> {
    let mut server = FastWebServer::new("0.0.0.0:7878", 12);
    bind![server, test_getter, test_getter2, mirror_response];
    server.run()
}

#[get("/test3")]
fn test_getter2(_request: HttpRequest) -> Vec<u8> {
    vec![62; 1000000]
}

#[get("/test")]
fn test_getter(_request: HttpRequest) -> Vec<u8> {
    "test".to_string().into_bytes()
}

#[post("/mirror")]
fn mirror_response(request: HttpRequest) -> Vec<u8> {
    request.body.into_bytes()
}
