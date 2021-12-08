use crate::FiniteBound;

#[derive(Clone, Copy, Debug)]
pub struct UpperBoundedRange<T> {
    pub(crate) end: FiniteBound<T>,
}

impl<T> From<std::ops::RangeTo<T>> for UpperBoundedRange<T> {
    fn from(r: std::ops::RangeTo<T>) -> Self {
        Self {
            end: FiniteBound::Excluded(r.end),
        }
    }
}

impl<T> From<std::ops::RangeToInclusive<T>> for UpperBoundedRange<T> {
    fn from(r: std::ops::RangeToInclusive<T>) -> Self {
        Self {
            end: FiniteBound::Included(r.end),
        }
    }
}

impl<T: Copy + Ord> UpperBoundedRange<T> {
    pub fn new(end: FiniteBound<T>) -> Self {
        Self { end }
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn contains(&self, t: T) -> bool {
        match self.end {
            FiniteBound::Excluded(x) => t < x,
            FiniteBound::Included(i) => t <= i,
        }
    }

    pub fn end_bound(&self) -> FiniteBound<T> {
        self.end
    }
}
