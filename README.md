# compiled-uuid

Parses UUIDs at compile time.

```rust
use uuid::Uuid;
use compiled_uuid::uuid;

// Success
let id: Uuid = uuid!("F9168C5E-CEB2-4FAA-B6BF-329BF39FA1E4");

// Failure
let id: Uuid = uuid!("F9168C5E-ZEB2-4FAA-B6BF-329BF39FA1E4");
```
