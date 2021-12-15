use crate::{Bound, BoundedRange, BoundedSet, Rangetools, UpperBoundedRange};

/// A set of ranges with a finite upper bound but no lower bound.
///
/// While an `UpperBoundedSet` can be constructed directly, it will most likely arise as a
/// result of one or more range operations.
/// ```
/// use rangetools::{UpperBoundedSet, Rangetools};
///
/// let s: UpperBoundedSet<_> = (10..20).union(..5);
/// ```
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct UpperBoundedSet<T> {
    /// Kept private to enforce the invariant that the ranges be non-empty and non-overlapping.
    pub(crate) upper_bounded_range: UpperBoundedRange<T>,
    pub(crate) ranges: BoundedSet<T>,
}

impl<T> From<UpperBoundedRange<T>> for UpperBoundedSet<T> {
    fn from(upper_bounded_range: UpperBoundedRange<T>) -> Self {
        Self {
            upper_bounded_range,
            ranges: BoundedSet::empty(),
        }
    }
}

impl<T: Copy + Ord> UpperBoundedSet<T> {
    fn defragment(&mut self) {
        while !self.ranges.is_empty() {
            if self
                .ranges
                .ranges
                .first()
                .unwrap()
                .intersection(self.upper_bounded_range)
                .is_empty()
            {
                return;
            } else {
                let range = self.ranges.ranges.remove(0);
                self.upper_bounded_range.end = self.upper_bounded_range.end.max(range.end);
            }
        }
    }
    pub(crate) fn add_range(&mut self, range: BoundedRange<T>) {
        self.ranges.add_range(range);
        self.defragment();
    }
    pub(crate) fn add_upper_bounded_range(&mut self, range: UpperBoundedRange<T>) {
        self.upper_bounded_range.end = Bound::max(self.upper_bounded_range.end, range.end);
        self.defragment();
    }
    pub(crate) fn add_set(&mut self, set: BoundedSet<T>) {
        for range in set.ranges {
            self.add_range(range);
        }
    }

    /// Returns true if the set contains `t`.
    ///
    /// # Example
    /// ```
    /// use rangetools::Rangetools;
    ///
    /// let s = (10..20).union(..5);
    /// assert!(s.contains(0));
    /// assert!(!s.contains(42));
    /// ```
    pub fn contains(&self, t: T) -> bool {
        self.upper_bounded_range.contains(t) || self.ranges.contains(t)
    }
}
