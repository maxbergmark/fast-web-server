mod fast_web_server;
use fast_web_server_types::RequestType;

pub use crate::fast_web_server::FastWebServer;


#[macro_export]
macro_rules! bind {
    ( $server:expr, $( $x:expr ),* ) => {
        $(
            $x.register(&mut $server);
        )*
    };
}

pub trait RegisterEndpoint {
    fn register(&self, server: &mut FastWebServer);
    fn request_type() -> RequestType;
    fn route() -> String;
}
