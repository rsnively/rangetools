use crate::{Rangetools, UpperBoundedSet};

impl<T> Rangetools for UpperBoundedSet<T> {
    fn is_empty(&self) -> bool {
        false
    }

    type Inner = Self;
    fn to_inner(self) -> Self::Inner {
        self
    }

    type Set = Self;
    fn to_set(self) -> Self::Set {
        self
    }
}
