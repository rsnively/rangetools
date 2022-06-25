/// Helper trait for performing range complement.
///
/// In most cases, users should import the `Rangetools` trait instead of this one, as it performs
/// the necessary forwarding.  See [`.complement()`](crate::Rangetools::complement) for more information.
pub trait RangeComplement<Output> {
    /// Returns a set of all the elements not in `self`.
    fn complement(self) -> Output;
}
