# Rangetools

Traits extending the Rust Range structs in std::ops

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rangetools = "0.1"
```

How to use in your crate:

```rust
use rangetools::Rangetools;
```

This provides new methods on all of the `std::ops` range types, as well as any
types introduced in this crate to manage the outputs of such methods.

```rust
let i = (0..5).intersection(3..);
assert!(i.contains(4));

let i2 = (0..5).intersection(5..10);
assert!(i2.is_empty());
```

Wherever possible (when the result is lower-bounded), the resulting
types of these operations implement `IntoIterator` so that more operations
can be performed on them.

```rust
let u1 = (1..3).union(5..7);
assert_eq!(u1.into_iter().collect::<Vec<_>>(), vec![1, 2, 5, 6]);

let u2 = (1..3).union(10..);
assert_eq!(u2.into_iter().take(5).collect::<Vec<_>>(), vec![1, 2, 10, 11, 12]);

let c = (1..3).complement();
let i = c.into_iter(); // Compiler error! The result has no lower bound
                       // and thus cannot be iterated over.
```

## Features

The **serde** feature provides derives for serde's Serialize and Deserialize traits.

## License

Licensed under the Apache License, Version 2.0 or the MIT license, at your option.
