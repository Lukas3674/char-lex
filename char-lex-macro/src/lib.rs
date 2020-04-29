#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

//! # CHAR-LEX-MACRO
//!
//! The `proc_macro_attribute` [`token`] for the [`char_lex`] crate.
//!
//! [`token`]: ./attr.token.html
//! [`char_lex`]: https://docs.rs/char-lex/0.1.0/char_lex/

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::fold::Fold;
use syn::{self, parse_macro_input};

/// The [`token`] attribute macro.
///
/// [`token`]: ./attr.token.html
#[proc_macro_attribute]
pub fn token(_unused: TokenStream, input: TokenStream) -> TokenStream {
    let item_enum = parse_macro_input!(input as syn::ItemEnum);

    let mut token_enum = TokenEnum::new();
    let item_enum = token_enum.fold_item_enum(item_enum);

    let mut token_stream = TokenStream::from(quote!(#item_enum));
    token_stream.extend(token_enum.to_token_stream());

    token_stream
}

enum CharExpr {
    Char(char),
    Chars(Vec<char>),
}

struct TokenEnum {
    ident: Option<syn::Ident>,
    exprs: Vec<(syn::Ident, CharExpr)>,
    fields: Vec<(syn::Ident, syn::Type)>,
}

impl TokenEnum {
    fn new() -> Self {
        Self {
            ident: None,
            exprs: Vec::new(),
            fields: Vec::new(),
        }
    }

    fn to_token_stream(&mut self) -> TokenStream {
        let ident = self.ident.as_ref().unwrap();

        let mut exprs = Vec::new();
        for expr in &self.exprs {
            exprs.push(parse_expr(expr));
        }

        let mut impls = Vec::new();
        for (_, ty) in &self.fields {
            impls.push(quote!(#ty: TokenTrait,))
        }
        let mut imp = quote!();
        if impls.len() > 0 {
            imp = quote!(where #(#impls)*);
        }

        let fields = parse_fields(&self.fields);

        TokenStream::from(quote!(
            impl TokenTrait for #ident {
                fn match_char(c: char) -> Option<Self> #imp {
                    match c {
                        #(#exprs)*
                        #(#fields)*
                    }
                }
            }
        ))
    }
}

impl Fold for TokenEnum {
    fn fold_item_enum(&mut self, mut i: syn::ItemEnum) -> syn::ItemEnum {
        self.ident = Some(i.ident.clone());
        for variant in &mut i.variants {
            handle_variant(variant, &mut self.exprs, &mut self.fields);
        }
        i
    }
}

fn handle_variant(
    variant: &mut syn::Variant,
    exprs: &mut Vec<(syn::Ident, CharExpr)>,
    fields: &mut Vec<(syn::Ident, syn::Type)>,
) {
    match &variant.fields {
        syn::Fields::Named(_) => panic!("A Token-Enum can't have named fields!"),
        syn::Fields::Unnamed(field) => {
            if field.unnamed.len() != 1 {
                panic!("A Token-Enum can only have one unnamed field!");
            }
            fields.push((variant.ident.clone(), field.unnamed[0].ty.clone()));
        }
        syn::Fields::Unit => {
            if let Some((_, expr)) = &variant.discriminant {
                match expr {
                    syn::Expr::Array(syn::ExprArray {
                        attrs: _,
                        bracket_token: _,
                        elems,
                    }) => {
                        let mut chars: Vec<char> = Vec::new();
                        for expr in elems {
                            match expr {
                                syn::Expr::Lit(syn::ExprLit{ attrs: _, lit: syn::Lit::Char(c) }) => chars.push(c.value()),
                                _ => panic!(
                                    "A Token-Enum can only be assigned to a single 'char' or multiple [chars]!"
                                ),
                            }
                        }
                        exprs.push((variant.ident.clone(), CharExpr::Chars(chars)));
                    }
                    syn::Expr::Lit(syn::ExprLit {
                        attrs: _,
                        lit: syn::Lit::Char(c),
                    }) => exprs.push((variant.ident.clone(), CharExpr::Char(c.value()))),
                    _ => panic!(
                        "A Token-Enum can only be assigned to a single 'char' or multiple [chars]!"
                    ),
                };
                variant.discriminant = None;
            } else {
                panic!("A Token-Enum with no fields needs to be assigned to a single 'char' or multiple [chars]!");
            }
        }
    };
}

fn parse_expr((ident, expr): &(syn::Ident, CharExpr)) -> impl ToTokens {
    match expr {
        CharExpr::Char(c) => quote!(#c => Some(Self::#ident),),
        CharExpr::Chars(chars) => quote!(#(#chars)|* => Some(Self::#ident),),
    }
}

fn parse_fields(fields: &Vec<(syn::Ident, syn::Type)>) -> Vec<impl ToTokens> {
    let mut output = vec![quote!(_ =>)];
    if fields.is_empty() {
        output.push(quote!(None,));
    } else {
        for (i, field) in fields.iter().enumerate() {
            if i != 0 {
                output.push(quote!(else));
            }
            output.push(parse_field(field).to_token_stream());
        }
        output.push(quote!(else { None }));
    }
    output
}

fn parse_field((ident, field): &(syn::Ident, syn::Type)) -> impl ToTokens {
    quote!(
        if let Some(field) = #field::match_char(c) {
            Some(Self::#ident(field))
        }
    )
}
