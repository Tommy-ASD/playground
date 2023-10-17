use std::str::FromStr;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemEnum, ItemStruct};

use crate::to_snake_case::ToSnakeCase;

mod to_snake_case;

#[proc_macro_derive(Setters)]
pub fn setters_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse(input).unwrap();

    match input {
        syn::Item::Struct(item_struct) => generate_setters_for_struct(item_struct),
        syn::Item::Enum(item_enum) => generate_setters_for_enum(item_enum),
        _ => panic!("Setters macro only supports structs and enums"),
    }
}

fn generate_setters_for_struct(item_struct: ItemStruct) -> proc_macro::TokenStream {
    let struct_name = &item_struct.ident;
    println!("Name: {struct_name}");
    let setters = item_struct.fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        println!("Field name: {field_name}");
        println!("Field type: {field_type:?}");
        println!("Field: {field:?}");
        let func_name = proc_macro2::TokenStream::from_str(&format!("set_{field_name}")).unwrap();

        let q = quote! {
            pub fn #func_name(&mut self, #field_name: #field_type) {
                self.#field_name = #field_name;
            }
        };
        println!("Q: {q}");
        q
    });

    let expanded = quote! {
        impl #struct_name {
            #(#setters)*
        }
    };

    expanded.into()
}

fn generate_setters_for_enum(item_enum: ItemEnum) -> proc_macro::TokenStream {
    let enum_name = &item_enum.ident;
    let setters = item_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let func_name =
            proc_macro2::TokenStream::from_str(&format!("set_{}", &variant_name).to_snake_case())
                .unwrap();
        quote! {
            pub fn #func_name(&mut self) {
                *self = #enum_name::#variant_name;
            }
        }
    });

    let expanded = quote! {
        impl #enum_name {
            #(#setters)*
        }
    };

    expanded.into()
}
