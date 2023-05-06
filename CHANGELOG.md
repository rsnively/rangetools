# Rangetools 0.1.4
## Added
- `std::ops::Index` and `std::ops::IndexMut` implementations matching those for the std::ops range types
    - implemented on ranges of `usize` for `String`, `str`, `CStr`, `CString`, `OsString`, `[T]`, and `Vec<T>` where applicable
    - unfortunately, the `std::slice::SliceIndex` trait is sealed so we couldn't do everything we wanted to
- conversion methods (`From` implementations) between this crate's range types and the std::ops range types where applicable:
    - `BoundedRange` -> `Range` and `RangeInclusive`
    - `EmptyRange` -> `Range` and `RangeInclusive`
    - `LowerBoundedRange` -> `RangeFrom`
    - `UnboundedRange` -> `RangeFull`
    - `UpperBoundedRange` -> `RangeTo` and `RangeToInclusive`
- `to_bound` methods on `UpperBound` and `LowerBound` with stern comment warning of pitfalls
- this changelog

# Rangetools 0.1.3
## Added
- optional `serde` dependency for deriving `Serialize` and `Deserialize` where applicable

# Rangetools 0.1.2
## Added
- `EmptyRange` type
- `RangeComplement` trait and `complement` method for the `Rangetools` trait
- `flipped` method for `Bound`
## Changed
- Iterators! Previously the range types implemented `Iterator` directly. Now they implement `IntoIterator` and have corresponding iterator types (`BoundedRangeIter`, `LowerBoundedRangeIter`, `BoundedSetIter`, and `LowerBoundedSetIter`)

# Rangetools 0.1.1
## Added
- `intersects` method for the `Rangetools` trait
## Changed
- Specialized `Bound` into `LowerBound` and `UpperBound` [#5](https://github.com/rsnively/rangetools/issues/5) (fixed bug in BoundedRange's iterator implementation [#4](https://github.com/rsnively/rangetools/issues/4))

# Rangetools 0.1.0
Initial Release