use crate::{Rangetools, UpperBoundedRange, UpperBoundedSet};

impl<T> Rangetools for UpperBoundedRange<T> {
    fn is_empty(&self) -> bool {
        false
    }

    type Inner = Self;
    fn to_inner(self) -> Self::Inner {
        self
    }

    type Set = UpperBoundedSet<T>;
    fn to_set(self) -> Self::Set {
        self.into()
    }
}
