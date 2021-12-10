use crate::{Bound, BoundedRange, BoundedSet, LowerBoundedRange, Rangetools};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct LowerBoundedSet<T> {
    pub(crate) ranges: BoundedSet<T>,
    pub(crate) lower_bounded_range: LowerBoundedRange<T>,
}

impl<T> From<LowerBoundedRange<T>> for LowerBoundedSet<T> {
    fn from(lower_bounded_range: LowerBoundedRange<T>) -> Self {
        Self {
            lower_bounded_range,
            ranges: BoundedSet::empty(),
        }
    }
}

impl<T: Copy + Ord + std::ops::Add<T, Output = T> + std::ops::AddAssign<T> + num_traits::One>
    Iterator for LowerBoundedSet<T>
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.ranges
            .next()
            .or_else(|| self.lower_bounded_range.next())
    }
}

impl<T: Copy + Ord> LowerBoundedSet<T> {
    fn defragment(&mut self) {
        while !self.ranges.is_empty() {
            if self
                .ranges
                .ranges
                .last()
                .unwrap()
                .intersection(self.lower_bounded_range)
                .is_empty()
            {
                return;
            } else {
                let range = self.ranges.ranges.pop().unwrap();
                self.lower_bounded_range.start = self.lower_bounded_range.start.min(range.start);
            }
        }
    }

    pub(crate) fn add_range(&mut self, range: BoundedRange<T>) {
        self.ranges.add_range(range);
        self.defragment();
    }
    pub(crate) fn add_lower_bounded_range(&mut self, range: LowerBoundedRange<T>) {
        self.lower_bounded_range.start = Bound::min(self.lower_bounded_range.start, range.start);
        self.defragment();
    }
    pub(crate) fn add_set(&mut self, set: BoundedSet<T>) {
        for range in set.ranges {
            self.add_range(range);
        }
    }

    pub fn contains(&self, t: T) -> bool {
        self.lower_bounded_range.contains(t) || self.ranges.contains(t)
    }
}
