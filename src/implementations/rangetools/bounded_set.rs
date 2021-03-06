use crate::{BoundedSet, Rangetools};

impl<T: Copy + Ord> Rangetools for BoundedSet<T> {
    fn is_empty(&self) -> bool {
        self.ranges.iter().all(|r| r.is_empty())
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
