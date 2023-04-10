use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use quote::ToTokens;
use syn::LitStr;
use syn::{ItemFn, parse_macro_input, parse_quote, Stmt, ItemStruct};
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

fn route(attr: TokenStream, item: TokenStream, request_type: TokenStream2) ->  TokenStream {
    let args = parse_macro_input!(attr as LitStr);
    // println!("{:?}", args);

    let fn_decl = parse_macro_input!(item as ItemFn);
    // let item_copy = fn_decl.clone();
    let name = fn_decl.clone().sig.ident;

    let struct_def: ItemStruct = parse_quote!(
        #[allow(non_camel_case_types)]
        struct #name;
    );

    let new_fn = quote!(

        #struct_def

        impl RegisterEndpoint for #name {
            fn register(&self, server: &mut FastWebServer) {
                #fn_decl
                server.bind(Self::request_type(), Self::route().as_str(), #name);
        
            }
        
            fn route() -> String {
                #args.to_string()
            }
        
            fn request_type() -> RequestType{
                #request_type
            }
        }
    );
    // println!("{}", new_fn.clone().to_string());
    // return item_copy.into_token_stream().into();
    new_fn.into()
}


#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    let request_type = quote!(RequestType::GET);
    route(attr, item, request_type)
    
}

#[proc_macro_attribute]
pub fn post(attr: TokenStream, item: TokenStream) -> TokenStream {
    let request_type = quote!(RequestType::POST);
    route(attr, item, request_type)

}
