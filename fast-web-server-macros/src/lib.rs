use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{ItemFn, parse_macro_input, parse_quote, Stmt};
// use fast_web_server_types::{HttpFn, HttpRequest, HttpResponse, RequestType};

// struct Route {
//     request_type: RequestType,
//     route: String,
//     func: HttpFn,
// }

// struct RouteHandler {
//     routes: Vec<Route>,
// }

// static route_handler: RouteHandler = RouteHandler {
//     routes: vec![],
// };

#[proc_macro_attribute]
pub fn add_hello_world(_attr: TokenStream, item: TokenStream) -> TokenStream {

    let mut fn_decl = parse_macro_input!(item as ItemFn);

    let mut prefix: Vec<Stmt> = parse_quote!(
        use std::time::Instant;
        let t0 = Instant::now();
    );
    let mut suffix = parse_quote!(
        let elapsed = t0.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    );

    prefix.append(&mut fn_decl.block.stmts);
    prefix.append(&mut suffix);

    let mut fn_decl2 = fn_decl.clone();
    fn_decl2.block.stmts = prefix;
    println!("{:?}", fn_decl2.clone().into_token_stream().to_string());
    fn_decl2.into_token_stream().into()
}

#[proc_macro_attribute]
pub fn fast_web_server_routes(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut fn_decl = parse_macro_input!(item as ItemFn);
    let server_declaration: Vec<Stmt> = parse_quote!(
        // self.bind(RequestType::GET, "/test", test_getter);
        // self.bind(RequestType::GET, "/other-test", other_test_getter);
        // self.bind(RequestType::POST, "/post-mirror", mirror_response);
    );

    for s in server_declaration {
        fn_decl.block.stmts.push(s);
    }
    println!("{:?}", fn_decl.clone().into_token_stream().to_string());
    fn_decl.into_token_stream().into()
}

#[proc_macro_attribute]
pub fn get(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let fn_decl = parse_macro_input!(item as ItemFn);
    fn_decl.into_token_stream().into()
}
/*
pub struct #name;
impl Register for #name {
    fn register(self) {
        #fn_decl
        Routes::register(#name);
    }
}

fn
 */