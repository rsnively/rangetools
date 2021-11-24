# Rangetools

Traits extending the Rust Range structs in std::ops

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rangetools = "0.1"
```

Use a trait in your crate:

```rust
use rangetools::RangeIntersection;

fn foo() {
    assert!((0..5).intersection(2..10).contains(3));
    assert!((0..5).intersection(5..10).is_empty());
    assert_eq!((..=2).intersection(1..).collect::<Vec<_>>(), vec![1, 2]);
}
```

## License

Licensed under the Apache License, Version 2.0 or the MIT license, at your option.
