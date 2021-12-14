use crate::{Bound, Step};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct LowerBoundedRange<T> {
    pub(crate) start: Bound<T>,
}

impl<T> From<std::ops::RangeFrom<T>> for LowerBoundedRange<T> {
    fn from(r: std::ops::RangeFrom<T>) -> Self {
        Self {
            start: Bound::Included(r.start),
        }
    }
}

impl<T: Copy + Ord> LowerBoundedRange<T> {
    pub fn new(start: Bound<T>) -> Self {
        Self { start }
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn contains(&self, t: T) -> bool {
        match self.start {
            Bound::Excluded(x) => t > x,
            Bound::Included(i) => t >= i,
        }
    }

    pub fn start_bound(&self) -> Bound<T> {
        self.start
    }
}

impl<T> Iterator for LowerBoundedRange<T>
where
    T: Copy + Ord + Step,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.start {
            Bound::Excluded(t) => {
                self.start = self.start.map(T::next);
                Some(t.next())
            }
            Bound::Included(t) => {
                self.start = self.start.map(T::next);
                Some(t)
            }
        }
    }
}
