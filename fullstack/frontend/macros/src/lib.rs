extern crate proc_macro;
use std::str::FromStr;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStatic};

#[proc_macro]
pub fn generate_state(input: TokenStream) -> TokenStream {
    let items = syn::parse_macro_input!(input as syn::File);
    let mut struct_fields = vec![];
    let mut with_block = vec![];
    let mut corrected_items = vec![];
    let mut getters = vec![];

    for item in items.items.iter() {
        if let syn::Item::Static(item_static) = item {
            let ident = &item_static.ident;
            let ident_lower =
                proc_macro2::TokenStream::from_str(&ident.to_string().to_lowercase()).unwrap();
            let getter_name =
                proc_macro2::TokenStream::from_str(&format!("get_{ident_lower}")).unwrap();
            corrected_items.push(quote! { pub static #ident: NodeRef = NodeRef::default(); });
            struct_fields.push(quote! { #ident_lower: NodeRef, });
            with_block.push(quote! { #ident_lower: #ident.with(|inner| inner.clone()), });
            getters.push(quote! { pub fn #getter_name() -> NodeRef {
                #ident.with(|inner| inner.clone())
            } })
        }
    }

    let generated = quote! {
        thread_local! {
            #(#corrected_items)*
        }

        pub struct State {
            #(#struct_fields),*
        }

        impl State {
            pub fn get() -> Self {
                Self {
                    #(#with_block)*
                }
            }
            #(#getters)*
        }
    };

    generated.into()
}
