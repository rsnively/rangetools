use crate::{Bound, LowerBound, Step};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::iter::FusedIterator;

/// A range only bounded below (either inclusive or exclusive).
///
/// Generalizes over [`std::ops::RangeFrom`] but also supports ranges with an exclusive lower bound.
///
/// While a `LowerBoundedRange` can be constructed directly, it will most likely
/// result from one or more range operations.
/// ```
/// use rangetools::{LowerBound, LowerBoundedRange, Rangetools};
///
/// let i = (5..).intersection(10..);
/// assert_eq!(i, LowerBoundedRange { start: LowerBound::included(10) });
/// ```
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct LowerBoundedRange<T> {
    /// The lower bound of the range (can be inclusive or exclusive).
    pub start: LowerBound<T>,
}

impl<T> From<std::ops::RangeFrom<T>> for LowerBoundedRange<T> {
    fn from(r: std::ops::RangeFrom<T>) -> Self {
        Self {
            start: LowerBound::included(r.start),
        }
    }
}

impl<T> From<LowerBoundedRange<T>> for std::ops::RangeFrom<T>
where
    T: Copy + Step,
{
    fn from(r: LowerBoundedRange<T>) -> Self {
        match r.start.to_bound() {
            Bound::Excluded(t) => Step::forward(t, 1)..,
            Bound::Included(t) => t..,
        }
    }
}

impl<T> IntoIterator for LowerBoundedRange<T>
where
    T: Copy + Step,
{
    type IntoIter = LowerBoundedRangeIter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        LowerBoundedRangeIter {
            current: match self.start {
                LowerBound(Bound::Excluded(t)) => Step::forward(t, 1),
                LowerBound(Bound::Included(t)) => t,
            },
        }
    }
}

impl<T: Copy + Ord> LowerBoundedRange<T> {
    /// Constructs a new `LowerBoundedRange` from a lower bound.
    ///
    /// # Example
    /// ```
    /// use rangetools::{LowerBound, LowerBoundedRange};
    ///
    /// let r = LowerBoundedRange::new(LowerBound::included(0));
    /// assert!(r.contains(5));
    /// ```
    pub fn new(start: LowerBound<T>) -> Self {
        Self { start }
    }

    /// Returns true if the range contains `t`.
    ///
    /// # Example
    /// ```
    /// use rangetools::Rangetools;
    ///
    /// let i = (5..).intersection(10..);
    /// assert!(i.contains(10));
    /// assert!(!i.contains(5));
    /// ```
    pub fn contains(&self, t: T) -> bool {
        match self.start.0 {
            Bound::Excluded(x) => t > x,
            Bound::Included(i) => t >= i,
        }
    }
}

/// An iterator over the values contained by a `LowerBoundedRange`.
///
/// Created by the `into_iter` method on `LowerBoundedRange` (provided by the [`std::iter::IntoIterator`] trait).
///
/// # Example
///
/// ```
/// # use rangetools::{LowerBoundedRange, LowerBoundedRangeIter};
/// let r: LowerBoundedRange<i32> = (0..).into();
/// let iter: LowerBoundedRangeIter<i32> = r.into_iter();
/// ```
#[derive(Clone, Debug)]
pub struct LowerBoundedRangeIter<T> {
    current: T,
}

impl<T> Iterator for LowerBoundedRangeIter<T>
where
    T: Copy + Step,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let t = self.current;
        self.current = Step::forward(self.current, 1);
        Some(t)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.current = Step::forward(self.current, n);
        self.next()
    }

    fn min(mut self) -> Option<Self::Item>
    where
        Self::Item: Ord,
    {
        self.next()
    }
}

impl<T> FusedIterator for LowerBoundedRangeIter<T> where T: Copy + Step {}
