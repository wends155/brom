use proc_macro::TokenStream;

#[proc_macro_derive(BromEntity, attributes(brom))]
pub fn derive_brom_entity(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
