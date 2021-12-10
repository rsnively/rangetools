use crate::{BoundedRange, BoundedSet, Rangetools};

impl<T: Copy + Ord> Rangetools for std::ops::Range<T> {
    fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    type Inner = BoundedRange<T>;
    fn to_inner(self) -> Self::Inner {
        self.into()
    }

    type Set = BoundedSet<T>;
    fn to_set(self) -> Self::Set {
        self.to_inner().to_set()
    }
}
