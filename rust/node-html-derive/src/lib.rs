//! Provides `ToHtml` derive macro for structs and enums in Stencila Schema

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Derive the `ToHtml` trait for a `struct` or an `enum`
#[proc_macro_derive(ToHtml)]
pub fn derive_to_html(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let tokens = match &input.data {
        Data::Struct(data) => derive_struct(&input, data),
        Data::Enum(data) => derive_enum(&input, data),
        Data::Union(..) => return proc_macro::TokenStream::new(),
    };

    proc_macro::TokenStream::from(tokens)
}

/// Derive the `ToHtml` trait for a `struct`
///
/// The implementation of methods is largely based on those for `IndexMap`.
fn derive_struct(input: &DeriveInput, data: &syn::DataStruct) -> TokenStream {
    let struct_name = &input.ident;

    let mut fields = TokenStream::new();
    for field in &data.fields {
        let field_name = &field.ident;
        let field_name_string = &field
            .ident
            .as_ref()
            .map(|ident| ident.to_string())
            .unwrap_or_default();
        if field_name_string == "r#type" {
            continue;
        }

        // TODO represent some fields as attributes, not all as children

        let field = {
            quote! {
                children.push(self.#field_name.to_html());
            }
        };
        fields.extend(field);
    }

    quote! {
        impl node_html::ToHtml for #struct_name {
            fn to_html(&self) -> String {
                use node_html::{attr, elem, name};

                let mut attrs = Vec::new();
                let mut children = Vec::new();

                #fields

                elem(&name(stringify!(#struct_name)), &attrs, &children)
            }
        }
    }
}

/// Derive the `ToHtml` trait for an `enum`
fn derive_enum(input: &DeriveInput, data: &syn::DataEnum) -> TokenStream {
    let enum_name = &input.ident;

    let mut cases = TokenStream::new();
    for variant in &data.variants {
        let variant_name = &variant.ident;
        let case = match &variant.fields {
            Fields::Named(..) | Fields::Unnamed(..) => quote! {
                Self::#variant_name(v) => v.to_html(),
            },
            Fields::Unit => quote! {
                Self::#variant_name => stringify!(#variant_name).to_string(),
            },
        };
        cases.extend(case)
    }

    quote! {
        impl node_html::ToHtml for #enum_name {
            fn to_html(&self) -> String {
                match self {
                    #cases
                }
            }
        }
    }
}
