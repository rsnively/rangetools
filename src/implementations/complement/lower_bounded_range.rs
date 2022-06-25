use crate::{LowerBoundedRange, RangeComplement, UpperBoundedRange};

impl<T> RangeComplement<UpperBoundedRange<T>> for LowerBoundedRange<T>
where
    T: Copy + Ord,
{
    fn complement(self) -> UpperBoundedRange<T> {
        UpperBoundedRange::new(self.start.0.flipped().into())
    }
}
