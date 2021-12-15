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

fn foo() {
    let i = (0..5).intersection(3..);
    assert!(i.contains(4));

    let u = (1..3).union(5..7);
    assert_eq!(u.collect::<Vec<_>>(), vec![1, 2, 5, 6]);
}
```

## License

Licensed under the Apache License, Version 2.0 or the MIT license, at your option.
