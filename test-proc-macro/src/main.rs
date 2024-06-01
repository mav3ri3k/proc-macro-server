#![feature(proc_macro_internals)]
extern crate proc_macro;
use proc_macro::TokenStream;

fn main() {
    let result = TokenStream::is_empty(&TokenStream::new());
    println!("{}", result);
    assert_eq!(result, true);

    static _DECLS: &[proc_macro::bridge::client::ProcMacro] =
        &[proc_macro::bridge::client::ProcMacro::bang("ans", ans)];
}

fn ans(_: TokenStream) -> TokenStream {
    "fn answer() -> u8{ 2 }".parse().unwrap()
}
