use crate::Rangetools;

/// Helper trait for performing range unions.
///
/// In most cases, users should import the `Rangetools` trait instead of this one, as it performs
/// the necessary forwarding.  See [`.union()`](crate::Rangetools::union) for more information.
pub trait RangeUnion<Rhs, RhsSet> {
    /// The output type of the union.
    type Output;
    /// Returns the set union of `self` and `other`.
    fn union(self, other: Rhs) -> Self::Output
    where
        Rhs: Rangetools<Set = RhsSet>;
}
