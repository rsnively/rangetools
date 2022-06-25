use crate::{LowerBoundedRange, RangeComplement, Rangetools};

impl<T> RangeComplement<LowerBoundedRange<T>> for std::ops::RangeToInclusive<T>
where
    T: Copy + Ord,
{
    fn complement(self) -> LowerBoundedRange<T> {
        RangeComplement::complement(self.to_inner())
    }
}
