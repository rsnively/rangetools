use crate::Rangetools;

/// Helper trait for performing range intersection.
///
/// In most cases, users should import the `Rangetools` trait instead of this one, as it performs
/// the necessary forwarding.  See [`.intersection()`](crate::Rangetools::intersection) for more information.
pub trait RangeIntersection<Rhs, RhsInner> {
    /// The output type of the intersection.
    type Output;
    /// Returns the set intersection of `self` and `other`.
    fn intersection(self, other: Rhs) -> Self::Output
    where
        Rhs: Rangetools<Inner = RhsInner>;
}
