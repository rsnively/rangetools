use crate::{LowerBoundedSet, RangeComplement, RangeIntersection, Rangetools, UpperBoundedSet};

impl<T> RangeComplement<LowerBoundedSet<T>> for UpperBoundedSet<T>
where
    T: Copy + Ord,
{
    fn complement(self) -> LowerBoundedSet<T> {
        self.ranges.ranges.into_iter().fold(
            RangeComplement::complement(self.upper_bounded_range).to_set(),
            |acc, range| RangeIntersection::intersection(acc, RangeComplement::complement(range)),
        )
    }
}
