use crate::{BoundedRange, BoundedSet, Rangetools};

impl<T: Copy + Ord> Rangetools for BoundedRange<T> {
    fn is_empty(&self) -> bool {
        self.start_bound() > self.end_bound()
    }

    type Inner = Self;
    fn to_inner(self) -> Self::Inner {
        self
    }

    type Set = BoundedSet<T>;
    fn to_set(self) -> Self::Set {
        self.into()
    }
}
