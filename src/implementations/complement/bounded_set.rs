use crate::{BoundedSet, RangeComplement, RangeIntersection, UnboundedSet};

impl<T> RangeComplement<UnboundedSet<T>> for BoundedSet<T>
where
    T: Copy + Ord,
{
    fn complement(self) -> UnboundedSet<T> {
        self.ranges
            .into_iter()
            .fold(UnboundedSet::Full, |acc, range| {
                acc.intersection(range.complement())
            })
    }
}
