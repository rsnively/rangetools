use crate::{BoundedRange, BoundedRangeIter, Rangetools, Step};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, iter::FusedIterator};

/// A set of ranges ultimately bounded both below and above.
///
/// While a `BoundedSet` can be constructed directly, it will most likely arise as a
/// result of one or more range operations.
/// ```
/// use rangetools::{BoundedSet, Rangetools};
///
/// let s: BoundedSet<_> = (0..3).union(3..5);
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct BoundedSet<T> {
    /// Kept private to enforce the invariant that the ranges be non-empty and non-overlapping.
    pub(crate) ranges: VecDeque<BoundedRange<T>>,
}

impl<T: Copy + Ord> From<BoundedRange<T>> for BoundedSet<T> {
    fn from(r: BoundedRange<T>) -> Self {
        if r.is_empty() {
            Self::empty()
        } else {
            Self { ranges: [r].into() }
        }
    }
}

impl<T> IntoIterator for BoundedSet<T>
where
    T: Copy + Ord + Step,
{
    type IntoIter = BoundedSetIter<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        BoundedSetIter {
            range_iters: self.ranges.into_iter().map(|r| r.into_iter()).collect(),
        }
    }
}

impl<T> BoundedSet<T> {
    /// Construct an empty `BoundedSet`.
    ///
    /// # Example
    /// ```
    /// use rangetools::{BoundedSet, Rangetools};
    ///
    /// let s = BoundedSet::empty();
    /// assert!(s.is_empty());
    /// assert!(!s.contains(5));
    /// ```
    pub fn empty() -> Self {
        Self {
            ranges: VecDeque::new(),
        }
    }
}

impl<T: Copy + Ord> BoundedSet<T> {
    pub(crate) fn add_range(&mut self, r: BoundedRange<T>) {
        if !r.is_empty() {
            if let Some(index) = self.ranges.iter().position(|range| range.intersects(r)) {
                let new_range = r.combine(&self.ranges[index]);
                self.ranges.remove(index);
                self.add_range(new_range);
            } else {
                let index = self
                    .ranges
                    .iter()
                    .position(|range| range.start > r.start)
                    .unwrap_or(self.ranges.len());
                self.ranges.insert(index, r);
            }
        }
    }
    pub(crate) fn add_set(&mut self, other: Self) {
        for range in other.ranges.into_iter() {
            self.add_range(range);
        }
    }

    /// Returns true if the set contains `t`.
    ///
    /// # Example
    /// ```
    /// use rangetools::Rangetools;
    ///
    /// let s = (1..5).union(10..20);
    /// assert!(s.contains(1));
    /// assert!(!s.contains(42));
    /// ```
    pub fn contains(&self, t: T) -> bool {
        self.ranges.iter().any(|r| r.contains(t))
    }
}

/// An iterator over the values contained by a `BoundedSet`.
///
/// Created by the `into_iter` method on `BoundedSet` (provided by the [`std::iter::IntoIterator`] trait).
///
/// # Example
///
/// ```
/// # use rangetools::{Rangetools, BoundedSet, BoundedSetIter};
/// let s: BoundedSet<i32> = (0..1).union(2..3);
/// let iter: BoundedSetIter<i32> = s.into_iter();
/// ```
#[derive(Clone, Debug)]
pub struct BoundedSetIter<T> {
    range_iters: VecDeque<BoundedRangeIter<T>>,
}

impl<T> Iterator for BoundedSetIter<T>
where
    T: Copy + Ord + Step,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while self
            .range_iters
            .front()
            .map(|i| i.len() == 0)
            .unwrap_or(false)
        {
            self.range_iters.pop_front();
        }
        self.range_iters.front_mut().map(|i| i.next()).flatten()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.range_iters.iter().map(|i| i.len()).sum();
        (size, Some(size))
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        self.len()
    }

    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        while self
            .range_iters
            .back()
            .map(|i| i.len() == 0)
            .unwrap_or(false)
        {
            self.range_iters.pop_back();
        }
        self.range_iters.back_mut().map(|i| i.last()).flatten()
    }

    fn nth(&mut self, mut n: usize) -> Option<Self::Item> {
        while let Some(len) = self.range_iters.front().map(|i| i.len()) {
            if len <= n {
                n -= len;
                self.range_iters.remove(0);
            } else {
                break;
            }
        }
        self.range_iters.front_mut().map(|i| i.nth(n)).flatten()
    }

    fn min(mut self) -> Option<Self::Item> {
        self.next()
    }

    fn max(mut self) -> Option<Self::Item> {
        self.next_back()
    }
}

impl<T> DoubleEndedIterator for BoundedSetIter<T>
where
    T: Copy + Ord + Step,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        while self
            .range_iters
            .back()
            .map(|i| i.len() == 0)
            .unwrap_or(false)
        {
            self.range_iters.pop_back();
        }
        self.range_iters.back_mut().map(|i| i.next_back()).flatten()
    }

    fn nth_back(&mut self, mut n: usize) -> Option<Self::Item> {
        while let Some(len) = self.range_iters.back_mut().map(|i| i.len()) {
            if len <= n {
                n -= len;
                self.range_iters.pop_back();
            } else {
                break;
            }
        }
        self.range_iters.back_mut().map(|i| i.nth_back(n)).flatten()
    }
}

impl<T> ExactSizeIterator for BoundedSetIter<T> where T: Copy + Ord + Step {}

impl<T> FusedIterator for BoundedSetIter<T> where T: Copy + Ord + Step {}
