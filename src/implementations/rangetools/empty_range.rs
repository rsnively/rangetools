use crate::{EmptyRange, Rangetools};

impl<T> Rangetools for EmptyRange<T> {
    fn is_empty(&self) -> bool {
        true
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
