mod http_request;
mod http_headers;
mod http_version;
mod start_line;
mod request_target;
mod http_response;
mod request_type;
mod status_line;
mod status_code;

pub use crate::http_request::HttpRequest;
pub use crate::http_headers::HttpHeaders;
pub use crate::http_version::HttpVersion;
// use crate::start_line::StartLine;
pub use crate::request_type::RequestType;
pub use crate::http_response::HttpResponse;
pub use crate::status_code::StatusCode;
pub use crate::status_line::StatusLine;



// pub type HttpFn = fn(HttpRequest) -> HttpResponse;
// #![feature(type_alias_impl_trait)]
pub type HttpFn = fn(HttpRequest) -> Vec<u8>;



pub trait Responder {
    fn transform(&self) -> Vec<u8>;
}

impl Responder for Vec<u8> {
    fn transform(&self) -> Vec<u8> {
        self.to_owned()
    }
}
