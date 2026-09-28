//! Implementation of the `clx!` macro.

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Expr, Result, Token,
};

/// A single class entry in the clx! macro.
enum ClassEntry {
    /// A simple class expression (e.g., "btn" or my_class_var)
    Simple(Expr),
    /// A conditional class (e.g., is_active => "active")
    Conditional { condition: Expr, class: Box<Expr> },
}

impl Parse for ClassEntry {
    fn parse(input: ParseStream) -> Result<Self> {
        let expr: Expr = input.parse()?;

        // Check if this is a conditional entry
        if input.peek(Token![=>]) {
            input.parse::<Token![=>]>()?;
            let class: Expr = input.parse()?;
            Ok(ClassEntry::Conditional {
                condition: expr,
                class: Box::new(class),
            })
        } else {
            Ok(ClassEntry::Simple(expr))
        }
    }
}

/// Input to the clx! macro - a comma-separated list of class entries.
struct ClxInput {
    entries: Vec<ClassEntry>,
}

impl Parse for ClxInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut entries = Vec::new();

        while !input.is_empty() {
            entries.push(input.parse()?);

            // Optional trailing comma
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(ClxInput { entries })
    }
}

pub fn clx_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ClxInput);

    let mut parts = Vec::new();

    for entry in input.entries {
        match entry {
            ClassEntry::Simple(expr) => {
                parts.push(quote! {
                    {
                        let class: &str = &#expr;
                        if !class.is_empty() {
                            classes.push(class.to_string());
                        }
                    }
                });
            }
            ClassEntry::Conditional { condition, class } => {
                let class = *class;
                parts.push(quote! {
                    if #condition {
                        let class: &str = &#class;
                        if !class.is_empty() {
                            classes.push(class.to_string());
                        }
                    }
                });
            }
        }
    }

    let expanded = quote! {
        {
            let mut classes: Vec<String> = Vec::new();
            #(#parts)*
            classes.join(" ")
        }
    };

    TokenStream::from(expanded)
}
