use crate::{Bound, BoundedRange, BoundedSet, Rangetools, UpperBoundedRange};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct UpperBoundedSet<T> {
    pub(crate) upper_bounded_range: UpperBoundedRange<T>,
    pub(crate) ranges: BoundedSet<T>,
}

impl<T> From<UpperBoundedRange<T>> for UpperBoundedSet<T> {
    fn from(upper_bounded_range: UpperBoundedRange<T>) -> Self {
        Self {
            upper_bounded_range,
            ranges: BoundedSet::empty(),
        }
    }
}

impl<T> UpperBoundedSet<T> {
    pub fn is_empty(&self) -> bool {
        false
    }
}

impl<T: Copy + Ord> UpperBoundedSet<T> {
    fn defragment(&mut self) {
        while !self.ranges.is_empty() {
            if self
                .ranges
                .ranges
                .first()
                .unwrap()
                .intersection(self.upper_bounded_range)
                .is_empty()
            {
                return;
            } else {
                let range = self.ranges.ranges.remove(0);
                self.upper_bounded_range.end = self.upper_bounded_range.end.max(range.end);
            }
        }
    }
    pub(crate) fn add_range(&mut self, range: BoundedRange<T>) {
        self.ranges.add_range(range);
        self.defragment();
    }
    pub(crate) fn add_upper_bounded_range(&mut self, range: UpperBoundedRange<T>) {
        self.upper_bounded_range.end = Bound::max(self.upper_bounded_range.end, range.end);
        self.defragment();
    }
    pub(crate) fn add_set(&mut self, set: BoundedSet<T>) {
        for range in set.ranges {
            self.add_range(range);
        }
    }

    pub fn contains(&self, t: T) -> bool {
        self.upper_bounded_range.contains(t) || self.ranges.contains(t)
    }
}
