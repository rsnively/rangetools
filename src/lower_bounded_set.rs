use crate::{BoundedRange, BoundedSet, LowerBound, LowerBoundedRange, Rangetools, Step};

/// A set of ranges with a finite lower bound but no upper bound.
///
/// While a `LowerBoundedSet` can be constructed directly, it will most likely arise as a
/// result of one or more range operations.
/// ```
/// use rangetools::{LowerBoundedSet, Rangetools};
///
/// let s: LowerBoundedSet<_> = (0..3).union(5..);
/// ```
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct LowerBoundedSet<T> {
    /// Kept private to enforce the invariant that the ranges be non-empty and non-overlapping.
    pub(crate) ranges: BoundedSet<T>,
    pub(crate) lower_bounded_range: LowerBoundedRange<T>,
}

impl<T> From<LowerBoundedRange<T>> for LowerBoundedSet<T> {
    fn from(lower_bounded_range: LowerBoundedRange<T>) -> Self {
        Self {
            lower_bounded_range,
            ranges: BoundedSet::empty(),
        }
    }
}

impl<T: Copy + Ord + Step> Iterator for LowerBoundedSet<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.ranges
            .next()
            .or_else(|| self.lower_bounded_range.next())
    }
}

impl<T: Copy + Ord> LowerBoundedSet<T> {
    fn defragment(&mut self) {
        while !self.ranges.is_empty() {
            if self
                .ranges
                .ranges
                .last()
                .unwrap()
                .intersection(self.lower_bounded_range)
                .is_empty()
            {
                return;
            } else {
                let range = self.ranges.ranges.pop().unwrap();
                self.lower_bounded_range.start = self.lower_bounded_range.start.min(range.start);
            }
        }
    }

    pub(crate) fn add_range(&mut self, range: BoundedRange<T>) {
        self.ranges.add_range(range);
        self.defragment();
    }
    pub(crate) fn add_lower_bounded_range(&mut self, range: LowerBoundedRange<T>) {
        self.lower_bounded_range.start =
            LowerBound::min(self.lower_bounded_range.start, range.start);
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
    /// let s = (1..5).union(10..);
    /// assert!(s.contains(42));
    /// assert!(!s.contains(0));
    /// ```
    pub fn contains(&self, t: T) -> bool {
        self.lower_bounded_range.contains(t) || self.ranges.contains(t)
    }
}
