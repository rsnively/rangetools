use crate::{EmptyRange, UnboundedRange, RangeComplement};

impl<T> RangeComplement<EmptyRange<T>> for UnboundedRange {
    fn complement(self) -> EmptyRange<T> {
        EmptyRange::new()
    }
}
