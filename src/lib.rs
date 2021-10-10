//! [![github-img]][github-url] [![crates-img]][crates-url] [![docs-img]][docs-url]
//! 
//! [github-url]: https://github.com/QnnOkabayashi/compiled-uuid
//! [crates-url]: https://crates.io/crates/compiled-uuid
//! [docs-url]: https://docs.rs/compiled-uuid/*/compiled_uuid
//! [github-img]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-img]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-img]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! Anywhere you're building [`Uuid`][uuid::Uuid]s from a string literal, you should use [`uuid`][compiled_uuid::uuid].
//! 
//! ## Overview
//! `compiled_uuid` exposes one macro called [`uuid`][compiled_uuid::uuid], which parses [`Uuid`][uuid::Uuid]s at compile time.
//!
//! When you write this:
//! ```rust
//! let id: Uuid = uuid!("F9168C5E-CEB2-4FAA-B6BF-329BF39FA1E4");
//! ```
//! It expands to:
//! ```rust
//! let id: Uuid = ::uuid::Uuid::from_bytes([
//!     249u8, 22u8, 140u8, 94u8, 206u8, 178u8, 79u8, 170u8, 182u8, 191u8, 50u8, 155u8, 243u8,
//!     159u8, 161u8, 228u8,
//! ]);
//! ```
//! 
//! [uuid::Uuid]: https://docs.rs/uuid/*/uuid/struct.Uuid.html
//! [compiled_uuid::uuid]: https://docs.rs/compiled-uuid/*/compiled_uuid/macro.uuid.html#

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use thiserror::Error;
use uuid::Uuid;

/// Parse [`Uuid`][uuid::Uuid]s from string literals at compile time.
/// ## Usage
/// This macro transforms the string literal representation of a [`Uuid`][uuid::Uuid] into the bytes representation,
/// raising a compilation error if it cannot properly be parsed.
/// 
/// ## Examples
/// Setting a global constant:
/// ```
/// # use uuid::Uuid;
/// # use compiled_uuid::uuid;
/// pub const SCHEMA_ATTR_CLASS: Uuid = uuid!("00000000-0000-0000-0000-ffff00000000");
/// pub const SCHEMA_ATTR_UUID: Uuid = uuid!("00000000-0000-0000-0000-ffff00000001");
/// pub const SCHEMA_ATTR_NAME: Uuid = uuid!("00000000-0000-0000-0000-ffff00000002");
/// ```
/// Defining a local variable:
/// ```
/// # use uuid::Uuid;
/// # use compiled_uuid::uuid;
/// let uuid: Uuid = uuid!("urn:uuid:F9168C5E-CEB2-4faa-B6BF-329BF39FA1E4");
/// ```
/// ## Compilation Failures
/// Invalid UUIDs are rejected:
/// ```ignore
/// # use uuid::Uuid;
/// # use compiled_uuid::uuid;
/// let uuid: Uuid = uuid!("F9168C5E-ZEB2-4FAA-B6BF-329BF39FA1E4");
/// ```
/// Provides the following compilation error:
/// ```txt
/// error: invalid character: expected an optional prefix of `urn:uuid:` followed by 0123456789abcdefABCDEF-, found Z at 9
///     |
///     |     let id: Uuid = uuid!("F9168C5E-ZEB2-4FAA-B6BF-329BF39FA1E4");
///     |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
/// ```
/// Tokens that aren't string literals are also rejected:
/// ```ignore
/// # use uuid::Uuid;
/// # use compiled_uuid::uuid;
/// let uuid_str: &str = "550e8400e29b41d4a716446655440000";
/// let uuid: Uuid = uuid!(uuid_str);
/// ```
/// Provides the following compilation error:
/// ```txt
/// error: expected string literal
///   |
///   |     let uuid: Uuid = uuid!(uuid_str);
///   |                            ^^^^^^^^
/// ```
/// 
/// [uuid::Uuid]: https://docs.rs/uuid/0.8.2/uuid/struct.Uuid.html
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
