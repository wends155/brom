use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

pub fn expand_openapi(struct_name: &Ident) -> TokenStream {
    let lower_name = struct_name.to_string().to_lowercase();
    let api_doc_name = format_ident!("{}Api", struct_name);
    let admin_api_mod_name = format_ident!("{}_admin_api", lower_name);
    let public_api_mod_name = format_ident!("{}_public_api", lower_name);
    let openapi_mod_name = format_ident!("{}_openapi_inner", lower_name);

    quote! {
        #[automatically_derived]
        #[allow(clippy::needless_for_each)]
        mod #openapi_mod_name {
            use ::brom::__private::utoipa as utoipa;

            #[derive(utoipa::OpenApi)]
            #[openapi(
                paths(
                    #admin_api_mod_name::list_handler,
                    #admin_api_mod_name::get_handler,
                    #admin_api_mod_name::create_handler,
                    #admin_api_mod_name::update_handler,
                    #admin_api_mod_name::delete_handler,
                    #public_api_mod_name::list_handler,
                    #public_api_mod_name::get_handler,
                ),
                components(schemas(#struct_name))
            )]
            pub struct #api_doc_name;
        }
        pub use #openapi_mod_name::#api_doc_name;
    }
}
