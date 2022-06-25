use crate::{EmptyRange, RangeComplement};

impl<T> RangeComplement<EmptyRange<T>> for std::ops::RangeFull {
    fn complement(self) -> EmptyRange<T> {
        EmptyRange::new()
    }
}
