use crate::{Rangetools, UpperBoundedRange, UpperBoundedSet};

impl<T: Copy + Ord> Rangetools for std::ops::RangeTo<T> {
    fn is_empty(&self) -> bool {
        false
    }
    type Inner = UpperBoundedRange<T>;
    type Set = UpperBoundedSet<T>;
    fn to_inner(self) -> Self::Inner {
        self.into()
    }
    fn to_set(self) -> Self::Set {
        self.to_inner().to_set()
    }
}
