use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

pub fn expand_openapi(struct_name: &Ident) -> TokenStream {
    let lower_name = struct_name.to_string().to_lowercase();
    let api_doc_name = format_ident!("{}Api", struct_name);
    let admin_api_mod_name = format_ident!("{}_admin_api", lower_name);
    let public_api_mod_name = format_ident!("{}_public_api", lower_name);
    let openapi_mod_name = format_ident!("{}_openapi_inner", lower_name);

    let admin_struct_name = format_ident!("{}Admin", struct_name);
    let public_struct_name = format_ident!("{}Public", struct_name);

    quote! {
        #[automatically_derived]
        #[allow(clippy::needless_for_each)]
        mod #openapi_mod_name {
            use ::brom::__private::utoipa as utoipa;

            #[derive(utoipa::OpenApi)]
            #[openapi(
                paths(
                    super::#admin_api_mod_name::list_handler,
                    super::#admin_api_mod_name::get_handler,
                    super::#admin_api_mod_name::create_handler,
                    super::#admin_api_mod_name::update_handler,
                    super::#admin_api_mod_name::delete_handler,
                    super::#public_api_mod_name::list_handler,
                    super::#public_api_mod_name::get_handler,
                ),
                components(schemas(super::#admin_struct_name, super::#public_struct_name))
            )]
            pub struct #api_doc_name;
        }
        pub use #openapi_mod_name::#api_doc_name;
    }
}
