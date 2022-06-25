use crate::{BoundedSet, RangeComplement, RangeIntersection, Rangetools, UnboundedSet};

impl<T> RangeComplement<BoundedSet<T>> for UnboundedSet<T>
where
    T: Copy + Ord,
{
    fn complement(self) -> BoundedSet<T> {
        match self {
            Self::Full => BoundedSet::empty(),
            Self::Piecewise(set) => {
                let starting_range = RangeIntersection::intersection(
                    RangeComplement::complement(set.lower_bounded_range),
                    RangeComplement::complement(set.upper_bounded_range),
                );
                set.ranges
                    .ranges
                    .into_iter()
                    .fold(starting_range.to_set(), |acc, range| {
                        RangeIntersection::intersection(acc, RangeComplement::complement(range))
                    })
            }
        }
    }
}
