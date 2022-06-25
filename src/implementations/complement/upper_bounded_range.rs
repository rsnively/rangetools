use crate::{LowerBoundedRange, RangeComplement, UpperBoundedRange};

impl<T> RangeComplement<LowerBoundedRange<T>> for UpperBoundedRange<T>
where
    T: Copy + Ord,
{
    fn complement(self) -> LowerBoundedRange<T> {
        LowerBoundedRange::new(self.end.0.flipped().into())
    }
}
