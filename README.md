# RMG Identifier

This crate provides the core identifier types used in the Recursive
Meta-Graph (RMG) engine. All identifiers are derived from a BLAKE3 hash of a
string label with a domain-separation prefix to prevent cross-type
collisions.

## Usage

```rust
use rmg_ident::{make_node_id, make_type_id};

let node_id = make_node_id("my-node");
let type_id = make_type_id("my-type");
```

## License

Licensed under the Apache License, Version 2.0.
