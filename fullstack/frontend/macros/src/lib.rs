extern crate proc_macro;
use std::str::FromStr;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn generate_state(input: TokenStream) -> TokenStream {
    println!("Got stream: {input}");
    let binding = input.to_string();
    let items = binding
        .split(",")
        .into_iter()
        .map(|part| part.trim())
        .collect::<Vec<&str>>();
    println!("Items: {items:?}");
    // let item_static = parse_macro_input!(input as ItemStatic);
    // println!("Parsed as file: {item_static:?}");
    let mut struct_fields = vec![];
    let mut with_block = vec![];
    let mut corrected_items = vec![];
    let mut getters = vec![];

    for item in items {
        if item.is_empty() {
            continue;
        }
        println!("Handling {item}");
        let ident = proc_macro2::TokenStream::from_str(&item.to_uppercase()).unwrap();
        let ident_lower = proc_macro2::TokenStream::from_str(&item.to_lowercase()).unwrap();
        let getter_name =
            proc_macro2::TokenStream::from_str(&format!("get_{ident_lower}")).unwrap();
        corrected_items.push(quote! { pub static #ident: NodeRef = NodeRef::default(); });
        struct_fields.push(quote! { #ident_lower: NodeRef });
        with_block.push(quote! { #ident_lower: #ident.with(|inner| inner.clone()), });
        getters.push(quote! { pub fn #getter_name() -> NodeRef {
            #ident.with(|inner| inner.clone())
        } })
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

    println!("Generated: {generated}");

    generated.into()
    // TokenStream::new()
}
