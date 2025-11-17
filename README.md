# RMG Identifier (`rmg-ident`)

This crate provides the core identifier types used in the Recursive
Meta-Graph (RMG) engine.

## Overview

All identifiers in RMG are derived from a BLAKE3 hash of a string label. To
prevent collisions between different types of identifiers (e.g., a node and a
type with the same label), each identifier is created with a domain-separation
prefix.

This crate provides:
- The core `Hash` type (a `[u8; 32]` array).
- Strongly-typed identifiers: `NodeId`, `TypeId`, `EdgeId`.
- A compact, in-process `CompactRuleId` for hot paths.
- Functions to create these identifiers from string labels.

## Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
rmg-ident = "0.1.0"
```

Then, you can use the functions to create identifiers:

```rust
use rmg_ident::{make_node_id, make_type_id, make_edge_id};

// Create a new NodeId
let node_id = make_node_id("my-node");

// Create a new TypeId
let type_id = make_type_id("my-type");

// Create a new EdgeId
let edge_id = make_edge_id("my-edge");
```

## API

### Structs
- `pub struct NodeId(pub Hash)`: A strongly-typed identifier for a node.
- `pub struct TypeId(pub Hash)`: A strongly-typed identifier for a type.
- `pub struct EdgeId(pub Hash)`: A strongly-typed identifier for an edge.
- `pub struct CompactRuleId(pub u32)`: A compact, process-local rule identifier.

### Functions
- `pub fn make_node_id(label: &str) -> NodeId`: Creates a new `NodeId` from a string label.
- `pub fn make_type_id(label: &str) -> TypeId`: Creates a new `TypeId` from a string label.
- `pub fn make_edge_id(label: &str) -> EdgeId`: Creates a new `EdgeId` from a string label.

## License

Licensed under the Apache License, Version 2.0. See `LICENSE` and `NOTICE` for details.