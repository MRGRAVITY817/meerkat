use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn controller(_: TokenStream, _input: TokenStream) -> TokenStream {
    quote::quote!().into()
}
