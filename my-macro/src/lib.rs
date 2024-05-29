#[proc_macro]
pub fn make_answer(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
