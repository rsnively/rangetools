use crate::{BoundedRange, BoundedSet, Rangetools};

impl<T: Copy + Ord> Rangetools for BoundedRange<T> {
    fn is_empty(&self) -> bool {
        self.start_bound() > self.end_bound()
    }
    type Inner = Self;
    type Set = BoundedSet<T>;
    fn to_inner(self) -> Self::Inner {
        self
    }
    fn to_set(self) -> Self::Set {
        self.into()
    }
}
