use crate::{RangeComplement, Rangetools, UnboundedSet};

impl<T> RangeComplement<UnboundedSet<T>> for std::ops::Range<T>
where
    T: Copy + Ord,
{
    fn complement(self) -> UnboundedSet<T> {
        RangeComplement::complement(self.to_inner())
    }
}
