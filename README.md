# compiled-uuid &emsp; [![Latest Version]][crates.io] ![MIT licensed][mit-badge]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[Latest Version]: https://img.shields.io/badge/crates.io-v0.1.0-blue
[crates.io]: https://crates.io/crates/compiled-uuid

Parse UUIDs at compile time.

```rust
use uuid::Uuid;
use compiled_uuid::uuid;

// Success
let id: Uuid = uuid!("F9168C5E-CEB2-4FAA-B6BF-329BF39FA1E4");

// Compile error, contains 'Z'
let id: Uuid = uuid!("F9168C5E-ZEB2-4FAA-B6BF-329BF39FA1E4");
```
