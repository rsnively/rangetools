use crate::{Bound, LowerBound, Rangetools, Step, UpperBound};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::iter::FusedIterator;

/// A range bounded both below and above (either inclusive or exclusive).
///
/// Generalizes over [`std::ops::Range`] and [`std::ops::RangeInclusive`] but also supports ranges
/// with an exclusive lower bound.
///
/// While a `BoundedRange` can be constructed from one of the above types, it will most likely
/// result from one or more range operations.
/// ```
/// use rangetools::{BoundedRange, LowerBound, Rangetools, UpperBound};
///
/// let i = (0..5).intersection(3..7);
/// assert_eq!(i, BoundedRange { start: LowerBound::included(3), end: UpperBound::excluded(5) });
/// ```
///
/// The range is empty if the start bound is greater than the end bound. Note that a range
/// with start bound `Bound::Excluded(3)` and end bound `Bound::Excluded(4)` is not considered
/// empty even though it doesn't contain any values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct BoundedRange<T> {
    /// The lower bound of the range (can be inclusive or exclusive).
    pub start: LowerBound<T>,
    /// The upper bound of the range (can be inclusive or exclusive).
    pub end: UpperBound<T>,
}

impl<T> From<std::ops::Range<T>> for BoundedRange<T> {
    fn from(r: std::ops::Range<T>) -> Self {
        Self {
            start: LowerBound::included(r.start),
            end: UpperBound::excluded(r.end),
        }
    }
}

impl<T> From<std::ops::RangeInclusive<T>> for BoundedRange<T> {
    fn from(r: std::ops::RangeInclusive<T>) -> Self {
        let (start, end) = r.into_inner();
        Self {
            start: LowerBound::included(start),
            end: UpperBound::included(end),
        }
    }
}

impl<T> From<BoundedRange<T>> for std::ops::Range<T>
where
    T: Copy + Step,
{
    fn from(r: BoundedRange<T>) -> Self {
        let start = match r.start.to_bound() {
            Bound::Excluded(t) => Step::forward(t, 1),
            Bound::Included(t) => t,
        };
        let end = match r.end.to_bound() {
            Bound::Excluded(t) => t,
            Bound::Included(t) => Step::forward(t, 1),
        };
        start..end
    }
}

impl<T> From<BoundedRange<T>> for std::ops::RangeInclusive<T>
where
    T: Copy + Step,
{
    fn from(r: BoundedRange<T>) -> Self {
        let start = match r.start.to_bound() {
            Bound::Excluded(t) => Step::forward(t, 1),
            Bound::Included(t) => t,
        };
        let end = match r.end.to_bound() {
            Bound::Excluded(t) => Step::backward(t, 1),
            Bound::Included(t) => t,
        };
        start..=end
    }
}

impl<T> IntoIterator for BoundedRange<T>
where
    T: Copy + Ord + Step,
{
    type IntoIter = BoundedRangeIter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        let current = match self.start {
            LowerBound(Bound::Excluded(t)) => Step::forward(t, 1),
            LowerBound(Bound::Included(t)) => t,
        };
        let last = match self.end {
            UpperBound(Bound::Excluded(t)) => Step::backward(t, 1),
            UpperBound(Bound::Included(t)) => t,
        };
        BoundedRangeIter { current, last }
    }
}

impl<T: Copy + Ord> BoundedRange<T> {
    /// Constructs a new `BoundedRange` from a lower bound and an upper bound.
    ///
    /// # Example
    /// ```
    /// use rangetools::{BoundedRange, LowerBound, UpperBound};
    ///
    /// let r = BoundedRange::new(LowerBound::included(0), UpperBound::included(10));
    /// assert!(r.contains(5));
    /// ```
    pub fn new(start: LowerBound<T>, end: UpperBound<T>) -> Self {
        Self { start, end }
    }

    /// Returns true if the range contains `t`.
    ///
    /// # Example
    /// ```
    /// use rangetools::Rangetools;
    ///
    /// let r = (1..10).intersection(5..7);
    /// assert!(r.contains(5));
    /// assert!(!r.contains(3));
    /// ```
    pub fn contains(&self, t: T) -> bool {
        let start_satisfied = match self.start.0 {
            Bound::Excluded(s) => t > s,
            Bound::Included(s) => t >= s,
        };
        let end_satisfied = match self.end.0 {
            Bound::Excluded(e) => t < e,
            Bound::Included(e) => t <= e,
        };
        start_satisfied && end_satisfied
    }

    pub(crate) fn combine(&self, other: &Self) -> Self {
        if other.is_empty() {
            return self.clone();
        }
        if self.is_empty() {
            return other.clone();
        }
        assert!(self.intersects(*other));
        BoundedRange::new(
            LowerBound::min(self.start, other.start),
            UpperBound::max(self.end, other.end),
        )
    }
}

/// An iterator over the values contained by a `BoundedRange`.
///
/// Created by the `into_iter` method on `BoundedRange` (provided by the [`std::iter::IntoIterator`] trait).
///
/// # Example
///
/// ```
/// # use rangetools::{BoundedRange, BoundedRangeIter};
/// let r: BoundedRange<i32> = (0..10).into();
/// let iter: BoundedRangeIter<i32> = r.into_iter();
/// ```
#[derive(Clone, Debug)]
pub struct BoundedRangeIter<T> {
    current: T,
    last: T,
}

impl<T> Iterator for BoundedRangeIter<T>
where
    T: Copy + Ord + Step,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.last {
            None
        } else {
            let t = self.current;
            self.current = Step::forward(self.current, 1);
            Some(t)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = Step::steps_between(&self.current, &self.last)
            .map(|steps| steps + 1)
            .unwrap_or_default();
        (size, Some(size))
    }

    fn count(self) -> usize {
        self.len()
    }

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        if self.current > self.last {
            None
        } else {
            Some(self.last)
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.current = Step::forward(self.current, n);
        self.next()
    }

    fn min(mut self) -> Option<Self::Item> {
        self.next()
    }

    fn max(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl<T> DoubleEndedIterator for BoundedRangeIter<T>
where
    T: Copy + Ord + Step,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current > self.last {
            None
        } else {
            let t = self.last;
            self.last = Step::backward(self.last, 1);
            Some(t)
        }
    }

    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        self.last = Step::backward(self.last, n);
        self.next_back()
    }
}

impl<T> ExactSizeIterator for BoundedRangeIter<T> where T: Copy + Ord + Step {}

impl<T> FusedIterator for BoundedRangeIter<T> where T: Copy + Ord + Step {}
