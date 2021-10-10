# compiled-uuid
[![github]][github-com] [![crates.io]][crates-io] [![docs.rs]][docs-rs]

[github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
[github-com]: https://github.com/QnnOkabayashi/compiled-uuid
[crates.io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
[crates-io]: https://crates.io/crates/compiled-uuid
[docs.rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
[docs-rs]: https://docs.rs/compiled-uuid/0.1.0/compiled_uuid/

Parse [`Uuid`](https://docs.rs/uuid/0.8.2/uuid/struct.Uuid.html)s at compile time.

## TL:DR
Anywhere you're building `Uuid`s from a string literal, you should use `compiled_uuid::uuid`.

## Motivation
If you want to use a fixed `Uuid` throughout your program and avoid parsing it multiple times, often you might use `lazy_static` to cache the `Uuid` after parsing the first time:
```rust
lazy_static! {
    pub static ref MY_UUID: Uuid = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
}
```
However, this method introduces overhead through parsing and `unwrap`ping at runtime.

`compiled_uuid::uuid`, on the other hand, provides a zero-cost runtime solution:
```rust
const MY_UUID: Uuid = uuid!("550e8400-e29b-41d4-a716-446655440000");
```

## Usage
`compiled_uuid` exposes one macro called `uuid`, which parses `Uuid`s at compile time. On success, it resolves to [`Uuid::from_bytes`](https://docs.rs/uuid/0.8.2/uuid/struct.Uuid.html#method.from_bytes), which cannot fail and has zero runtime cost.

When you write this:
```rust
let id: Uuid = uuid!("F9168C5E-CEB2-4FAA-B6BF-329BF39FA1E4");
```
It expands to this:
```rust
let id: Uuid = ::uuid::Uuid::from_bytes([
	249u8, 22u8, 140u8, 94u8, 206u8, 178u8, 79u8, 170u8, 182u8, 191u8, 50u8, 155u8, 243u8,
	159u8, 161u8, 228u8,
]);
```
If the UUID cannot be parsed successfully:
```rust
let id: Uuid = uuid!("F9168C5E-ZEB2-4FAA-B6BF-329BF39FA1E4");
```
Then a compiler error is raised:
```
error: invalid character: expected an optional prefix of `urn:uuid:` followed by 0123456789abcdefABCDEF-, found Z at 9
   |
   |     let id: Uuid = uuid!("F9168C5E-ZEB2-4FAA-B6BF-329BF39FA1E4");
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

## License
`compiled-uuid` is open-source software, distributed under the MIT license.
