use crate::{LowerBoundedSet, RangeComplement, RangeIntersection, Rangetools, UpperBoundedSet};

impl<T> RangeComplement<UpperBoundedSet<T>> for LowerBoundedSet<T>
where
    T: Copy + Ord,
{
    fn complement(self) -> UpperBoundedSet<T> {
        self.ranges.ranges.into_iter().fold(
            RangeComplement::complement(self.lower_bounded_range).to_set(),
            |acc, range| RangeIntersection::intersection(acc, RangeComplement::complement(range)),
        )
    }
}
