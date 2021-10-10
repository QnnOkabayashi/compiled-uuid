//! Parse `Uuid`s at compile time.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use thiserror::Error;
use uuid::Uuid;

/// # compiled-uuid
/// Parse [`Uuid`](https://docs.rs/uuid/0.8.2/uuid/struct.Uuid.html)s at compile time.
///
/// ## TL:DR
/// Anywhere you're building `Uuid`s from a string literal, you should use `compiled_uuid::uuid`.
///
/// ## Usage
/// `compiled_uuid` exposes one macro called `uuid`, which parses `Uuid`s at compile time. On success, it resolves to [`Uuid::from_bytes`](https://docs.rs/uuid/0.8.2/uuid/struct.Uuid.html#method.from_bytes), which cannot fail and has zero runtime cost.
///
/// When you write this:
/// ```
/// # use uuid::Uuid;
/// # use compiled_uuid::uuid;
/// let id: Uuid = uuid!("F9168C5E-CEB2-4FAA-B6BF-329BF39FA1E4");
/// ```
/// It expands to this:
/// ```
/// # use uuid::Uuid;
/// # use compiled_uuid::uuid;
/// let id: Uuid = ::uuid::Uuid::from_bytes([
///     249u8, 22u8, 140u8, 94u8, 206u8, 178u8, 79u8, 170u8, 182u8, 191u8, 50u8, 155u8, 243u8,
///     159u8, 161u8, 228u8,
/// ]);
/// ```
/// If the UUID cannot be parsed successfully:
/// ```ignore
/// let id: Uuid = uuid!("F9168C5E-ZEB2-4FAA-B6BF-329BF39FA1E4");
/// ```
/// Then a compiler error is raised:
/// ```txt
/// error: invalid character: expected an optional prefix of `urn:uuid:` followed by 0123456789abcdefABCDEF-, found Z at 9
/// ```
/// ```ignore
///     |
///     |     let id: Uuid = uuid!("F9168C5E-ZEB2-4FAA-B6BF-329BF39FA1E4");
///     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
/// ```
/// ## License
/// `compiled-uuid` is open-source software, distributed under the MIT license.
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

#[derive(Debug, Error)]
enum Error {
    #[error("expected string literal")]
    NonLiteral(#[from] syn::Error),
    #[error("expected string literal")]
    NonStringLiteral,
    #[error("{0}")]
    UuidParse(#[from] uuid::Error),
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
