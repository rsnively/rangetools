use crate::{Bound, Rangetools, Step};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BoundedRange<T> {
    pub(crate) start: Bound<T>,
    pub(crate) end: Bound<T>,
}

impl<T> From<std::ops::Range<T>> for BoundedRange<T> {
    fn from(r: std::ops::Range<T>) -> Self {
        Self {
            start: Bound::Included(r.start),
            end: Bound::Excluded(r.end),
        }
    }
}

impl<T> From<std::ops::RangeInclusive<T>> for BoundedRange<T> {
    fn from(r: std::ops::RangeInclusive<T>) -> Self {
        let (start, end) = r.into_inner();
        Self {
            start: Bound::Included(start),
            end: Bound::Included(end),
        }
    }
}

impl<T: Copy + Ord> BoundedRange<T> {
    pub fn new(start: Bound<T>, end: Bound<T>) -> Self {
        Self { start, end }
    }

    pub fn start_bound(&self) -> Bound<T> {
        self.start
    }
    pub fn end_bound(&self) -> Bound<T> {
        self.end
    }

    pub fn contains(&self, t: T) -> bool {
        let start_satisfied = match self.start {
            Bound::Excluded(s) => t > s,
            Bound::Included(s) => t >= s,
        };
        let end_satisfied = match self.end {
            Bound::Excluded(e) => t < e,
            Bound::Included(e) => t <= e,
        };
        start_satisfied && end_satisfied
    }

    pub fn combine(&self, other: &Self) -> Self {
        if other.is_empty() {
            return self.clone();
        }
        if self.is_empty() {
            return other.clone();
        }
        assert!(!self.disjoint(*other));
        BoundedRange::new(
            Bound::min(self.start_bound(), other.start_bound()),
            Bound::max(self.end_bound(), other.end_bound()),
        )
    }
}

impl<T> Iterator for BoundedRange<T>
where
    T: Copy + Ord + Step,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.start {
            Bound::Included(t) => {
                if self.start > self.end {
                    None
                } else {
                    self.start = self.start.map(T::next);
                    Some(t)
                }
            }
            Bound::Excluded(t) => {
                if self.start >= self.end {
                    None
                } else {
                    self.start = self.start.map(T::next);
                    Some(t.next())
                }
            }
        }
    }
}
