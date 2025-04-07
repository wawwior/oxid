use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn hello(tokens: TokenStream) -> TokenStream {
    let _ = tokens;
    quote! {
        println!("hello from macro!");
    }
    .into()
}
