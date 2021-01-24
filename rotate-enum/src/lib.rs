use core::panic;

use proc_macro::TokenStream;
use proc_macro2::{fallback::unforce, Span};
use quote::quote;
use syn::{
    parse_macro_input, punctuated::Punctuated, AngleBracketedGenericArguments, Data, DeriveInput,
    Field, Fields, GenericArgument, Ident, Path, PathArguments, PathSegment, Token, Type, TypePath,
};

#[proc_macro_derive(RotateEnum)]
pub fn rotate_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    println!("input: {:#?}", input);
    let name = input.ident;
    println!("name: {:?}", name.to_string());

    let variants = if let Data::Enum(data) = &input.data {
        data.variants.iter().map(|v| v).collect::<Vec<_>>()
    } else {
        panic!("ehh");
    };

    let prepend = |ident: &Ident| -> Path {
        let mut punct = Punctuated::<PathSegment, Token![::]>::new();
        punct.push(PathSegment {
            ident: name.clone(),
            arguments: PathArguments::None,
        });
        punct.push(PathSegment {
            ident: ident.clone(),
            arguments: PathArguments::None,
        });
        Path {
            leading_colon: None,
            segments: punct,
        }
    };

    let prepended_variants = variants
        .iter()
        .map(|v| prepend(&v.ident))
        .collect::<Vec<_>>();
    println!("prepended: {:#?}", prepended_variants);

    let nexts = variants
        .iter()
        .skip(1)
        .chain(variants.iter().take(1))
        .map(|v| (&v.ident))
        .collect::<Vec<_>>();

    let prevs = variants
        .iter()
        .take(variants.len() - 1)
        .chain(std::iter::once(variants.last().unwrap()))
        .map(|v| (&v.ident))
        .collect::<Vec<_>>();

    println!("nexts: {:#?}", nexts);

    let tokens = quote! {
        impl #name{
            pub fn next(self) -> Self {
                match self {
                    #(Self::#variants => Self::#nexts, )*
                }
            }
            pub fn prev(self) -> Self {
                match self {
                    #(Self::#nexts => Self::#variants,)*
                }
            }
        }
    };

    println!("tokens: {}", tokens);

    tokens.into()
}
