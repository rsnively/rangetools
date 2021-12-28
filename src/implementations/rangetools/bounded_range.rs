use crate::{Bound, BoundedRange, BoundedSet, Rangetools};

impl<T: Copy + Ord> Rangetools for BoundedRange<T> {
    fn is_empty(&self) -> bool {
        match (self.start.0, self.end.0) {
            (Bound::Included(start), Bound::Included(end)) => start > end,
            (Bound::Excluded(start), Bound::Included(end))
            | (Bound::Included(start), Bound::Excluded(end))
            | (Bound::Excluded(start), Bound::Excluded(end)) => start >= end,
        }
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
