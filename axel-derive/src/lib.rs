use proc_macro::TokenStream;

#[proc_macro_derive(Controller, attributes(scope))]
pub fn derive_controller(token_stream: TokenStream) -> TokenStream {
    TokenStream::new()
}
