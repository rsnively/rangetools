use crate::{RangeComplement, Rangetools, UpperBoundedRange};

impl<T> RangeComplement<UpperBoundedRange<T>> for std::ops::RangeFrom<T>
where
    T: Copy + Ord,
{
    fn complement(self) -> UpperBoundedRange<T> {
        RangeComplement::complement(self.to_inner())
    }
}
