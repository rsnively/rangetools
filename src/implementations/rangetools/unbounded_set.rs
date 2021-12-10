use crate::{Rangetools, UnboundedSet};

impl<T> Rangetools for UnboundedSet<T> {
    fn is_empty(&self) -> bool {
        false
    }
    type Inner = Self;
    type Set = Self;
    fn to_inner(self) -> Self::Inner {
        self
    }
    fn to_set(self) -> Self::Set {
        self
    }
}
