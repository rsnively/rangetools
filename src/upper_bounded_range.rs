use crate::Bound;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct UpperBoundedRange<T> {
    pub(crate) end: Bound<T>,
}

impl<T> From<std::ops::RangeTo<T>> for UpperBoundedRange<T> {
    fn from(r: std::ops::RangeTo<T>) -> Self {
        Self {
            end: Bound::Excluded(r.end),
        }
    }
}

impl<T> From<std::ops::RangeToInclusive<T>> for UpperBoundedRange<T> {
    fn from(r: std::ops::RangeToInclusive<T>) -> Self {
        Self {
            end: Bound::Included(r.end),
        }
    }
}

impl<T: Copy + Ord> UpperBoundedRange<T> {
    pub fn new(end: Bound<T>) -> Self {
        Self { end }
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn contains(&self, t: T) -> bool {
        match self.end {
            Bound::Excluded(x) => t < x,
            Bound::Included(i) => t <= i,
        }
    }

    pub fn end_bound(&self) -> Bound<T> {
        self.end
    }
}
