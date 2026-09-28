//! Implementation of the `WriteCSSVars` derive macro.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Convert snake_case to camelCase for CSS variable names.
fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

pub fn write_css_vars_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return syn::Error::new_spanned(
                    &input,
                    "WriteCSSVars only supports structs with named fields",
                )
                .to_compile_error()
                .into()
            }
        },
        _ => {
            return syn::Error::new_spanned(&input, "WriteCSSVars only supports structs")
                .to_compile_error()
                .into()
        }
    };

    let css_var_entries: Vec<_> = fields
        .iter()
        .filter_map(|field| {
            let field_name = field.ident.as_ref()?;
            let field_name_str = field_name.to_string();
            let css_var_name = format!("--{}", to_camel_case(&field_name_str));

            Some(quote! {
                (
                    #css_var_name,
                    self.#field_name.to_string()
                )
            })
        })
        .collect();

    let expanded = quote! {
        impl #name {
            /// Get all CSS variable definitions as key-value pairs.
            pub fn css_vars(&self) -> Vec<(&'static str, String)> {
                vec![
                    #(#css_var_entries),*
                ]
            }

            /// Generate CSS variable declarations as a string.
            pub fn to_css_string(&self) -> String {
                self.css_vars()
                    .into_iter()
                    .map(|(k, v)| format!("{}: {};", k, v))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
        }
    };

    TokenStream::from(expanded)
}
