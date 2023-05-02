use crate::{EmptyRange, RangeComplement, UnboundedRange};

impl<T> RangeComplement<UnboundedRange> for EmptyRange<T> {
    fn complement(self) -> UnboundedRange {
        UnboundedRange
    }
}
