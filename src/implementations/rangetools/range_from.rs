use crate::{LowerBoundedRange, LowerBoundedSet, Rangetools};

impl<T: Copy + Ord> Rangetools for std::ops::RangeFrom<T> {
    fn is_empty(&self) -> bool {
        false
    }

    type Inner = LowerBoundedRange<T>;
    fn to_inner(self) -> Self::Inner {
        self.into()
    }

    type Set = LowerBoundedSet<T>;
    fn to_set(self) -> Self::Set {
        self.to_inner().to_set()
    }
}
