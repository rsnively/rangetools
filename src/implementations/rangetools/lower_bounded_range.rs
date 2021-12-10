use crate::{LowerBoundedRange, LowerBoundedSet, Rangetools};

impl<T> Rangetools for LowerBoundedRange<T> {
    fn is_empty(&self) -> bool {
        false
    }

    type Inner = Self;
    fn to_inner(self) -> Self::Inner {
        self
    }

    type Set = LowerBoundedSet<T>;
    fn to_set(self) -> Self::Set {
        self.into()
    }
}
