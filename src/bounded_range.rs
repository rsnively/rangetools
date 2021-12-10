use crate::{Bound, Rangetools};

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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum RangeRelation {
    Equal,
    Disjoint,
    Contains,
    Contained,
    Overlap,
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

    pub fn relation(&self, other: &Self) -> RangeRelation {
        if other.is_empty() {
            RangeRelation::Contains
        } else if self.is_empty() {
            RangeRelation::Contained
        } else if self == other {
            RangeRelation::Equal
        } else if self.end_bound() < other.start_bound() || other.end_bound() < self.start_bound() {
            RangeRelation::Disjoint
        } else if self.start_bound() <= other.start_bound() && self.end_bound() >= other.end_bound()
        {
            RangeRelation::Contains
        } else if self.start_bound() >= other.start_bound() && self.end_bound() <= other.end_bound()
        {
            RangeRelation::Contained
        } else {
            RangeRelation::Overlap
        }
    }

    pub fn combine(&self, other: &Self) -> Self {
        if other.is_empty() {
            return self.clone();
        }
        if self.is_empty() {
            return other.clone();
        }
        assert!(self.relation(other) != RangeRelation::Disjoint);
        BoundedRange::new(
            Bound::min(self.start_bound(), other.start_bound()),
            Bound::max(self.end_bound(), other.end_bound()),
        )
    }
}

impl<T> Iterator for BoundedRange<T>
where
    T: Copy + Ord + std::ops::AddAssign<T> + std::ops::Add<T, Output = T> + num_traits::One,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match (self.start, self.end) {
            (Bound::Included(start), Bound::Included(end)) => {
                if start > end {
                    None
                } else {
                    self.start += T::one();
                    Some(start)
                }
            }
            (Bound::Included(start), Bound::Excluded(end)) => {
                if start >= end {
                    None
                } else {
                    self.start += T::one();
                    Some(start)
                }
            }
            (Bound::Excluded(start), Bound::Included(end)) => {
                if start >= end {
                    None
                } else {
                    self.start += T::one();
                    Some(self.start.t())
                }
            }
            (Bound::Excluded(start), Bound::Excluded(end)) => {
                if start + T::one() >= end {
                    None
                } else {
                    self.start += T::one();
                    Some(self.start.t())
                }
            }
        }
    }
}
