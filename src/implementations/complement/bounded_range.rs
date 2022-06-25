use crate::{
    BoundedRange, LowerBoundedRange, RangeComplement, Rangetools, UnboundedSet, UpperBoundedRange,
};

impl<T> RangeComplement<UnboundedSet<T>> for BoundedRange<T>
where
    T: Copy + Ord,
{
    fn complement(self) -> UnboundedSet<T> {
        UpperBoundedRange::new(self.start.0.flipped().into())
            .union(LowerBoundedRange::new(self.end.0.flipped().into()))
    }
}
