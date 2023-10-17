use std::str::FromStr;

use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemEnum, ItemStruct};

use crate::to_snake_case::ToSnakeCase;

pub(crate) fn generate_getters_for_struct(item_struct: ItemStruct) -> proc_macro::TokenStream {
    let struct_name = &item_struct.ident;
    let getters = item_struct.fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let func_name = proc_macro2::TokenStream::from_str(&format!("get_{field_name}")).unwrap();

        let q = quote! {
            pub fn #func_name<'a>(&'a self) -> &'a #field_type {
                &self.#field_name
            }
        };
        q
    });

    let expanded = quote! {
        impl #struct_name {
            #(#getters)*
        }
    };

    expanded.into()
}

pub(crate) fn generate_mut_getters_for_struct(item_struct: ItemStruct) -> proc_macro::TokenStream {
    let struct_name = &item_struct.ident;
    let getters = item_struct.fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let func_name =
            proc_macro2::TokenStream::from_str(&format!("get_{field_name}_mut")).unwrap();

        let q = quote! {
            pub fn #func_name<'a>(&'a mut self) -> &'a mut #field_type {
                &mut self.#field_name
            }
        };
        q
    });

    let expanded = quote! {
        impl #struct_name {
            #(#getters)*
        }
    };

    expanded.into()
}

fn generate_getters_for_enum(item_enum: ItemEnum) -> proc_macro::TokenStream {
    let enum_name = &item_enum.ident;
    let getters = item_enum.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let func_name =
            proc_macro2::TokenStream::from_str(&format!("get_{}", &variant_name).to_snake_case())
                .unwrap();
        quote! {
            pub fn #func_name(&mut self) {
                *self = #enum_name::#variant_name;
            }
        }
    });

    let expanded = quote! {
        impl #enum_name {
            #(#getters)*
        }
    };

    expanded.into()
}
