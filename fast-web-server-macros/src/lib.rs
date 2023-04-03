use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{ItemFn, parse_macro_input, parse_quote, Stmt};


#[proc_macro_attribute]
pub fn _add_hello_world(_attr: TokenStream, item: TokenStream) -> TokenStream {

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