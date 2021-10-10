use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
enum Error {
    #[error("expected string literal")]
    NonLiteral(#[from] syn::Error),
    #[error("expected string literal")]
    NonStringLiteral,
    #[error("{0}")]
    UuidParse(#[from] uuid::Error),
}

#[proc_macro]
pub fn uuid(input: TokenStream) -> TokenStream {
    build_uuid(input.clone()).unwrap_or_else(|e| {
        let msg = format!("{}", e);
        TokenStream::from(quote_spanned! {
            TokenStream2::from(input).span() =>
            compile_error!(#msg)
        })
    })
}

fn build_uuid(input: TokenStream) -> Result<TokenStream, Error> {
    let uuid = match syn::parse::<syn::Lit>(input)? {
        syn::Lit::Str(ref literal) => literal.value(),
        _ => return Err(Error::NonStringLiteral),
    };

    let uuid = Uuid::parse_str(&uuid)?;

    let tokens = uuid
        .as_bytes()
        .iter()
        .map(|byte| {
            quote! {
                #byte,
            }
        })
        .collect::<TokenStream2>();

    Ok(quote! {::uuid::Uuid::from_bytes([#tokens])}.into())
}
