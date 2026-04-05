use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

pub fn expand_openapi(struct_name: &Ident) -> TokenStream {
    let lower_name = struct_name.to_string().to_lowercase();
    let api_doc_name = format_ident!("{}Api", struct_name);
    let api_mod_name = format_ident!("{}_api", lower_name);
    let openapi_mod_name = format_ident!("{}_openapi_inner", lower_name);

    quote! {
        #[automatically_derived]
        mod #openapi_mod_name {
            use super::*;
            // Alias brom_server::utoipa as utoipa so the derive macro can find it.
            use ::brom_server::utoipa;

            #[derive(utoipa::OpenApi)]
            #[openapi(
                paths(
                    #api_mod_name::list_handler,
                    #api_mod_name::get_handler,
                    #api_mod_name::create_handler,
                    #api_mod_name::update_handler,
                    #api_mod_name::delete_handler,
                ),
                components(schemas(#struct_name))
            )]
            pub struct #api_doc_name;
        }
        pub use #openapi_mod_name::#api_doc_name;
    }
}
