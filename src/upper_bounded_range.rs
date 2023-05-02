use crate::{Bound, Step, UpperBound};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A range only bounded above (either inclusive or exclusive).
///
/// Generalizes over [`std::ops::RangeTo`] and [`std::ops::RangeToInclusive`].
///
/// While an `UpperoundedRange` can be constructed directly, it will most likely
/// result from one or more range operations.
/// ```
/// use rangetools::{Rangetools, UpperBound, UpperBoundedRange};
///
/// let i = (..5).intersection(..=3);
/// assert_eq!(i, UpperBoundedRange { end: UpperBound::included(3) });
/// ```
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct UpperBoundedRange<T> {
    /// The upper bound of the range (can be inclusive or exclusive).
    pub end: UpperBound<T>,
}

impl<T> From<std::ops::RangeTo<T>> for UpperBoundedRange<T> {
    fn from(r: std::ops::RangeTo<T>) -> Self {
        Self {
            end: UpperBound::excluded(r.end),
        }
    }
}

impl<T> From<std::ops::RangeToInclusive<T>> for UpperBoundedRange<T> {
    fn from(r: std::ops::RangeToInclusive<T>) -> Self {
        Self {
            end: UpperBound::included(r.end),
        }
    }
}

impl<T> From<UpperBoundedRange<T>> for std::ops::RangeTo<T>
where
    T: Copy + Step,
{
    fn from(r: UpperBoundedRange<T>) -> Self {
        match r.end.to_bound() {
            Bound::Excluded(t) => ..t,
            Bound::Included(t) => ..Step::forward(t, 1),
        }
    }
}

impl<T> From<UpperBoundedRange<T>> for std::ops::RangeToInclusive<T>
where
    T: Copy + Step,
{
    fn from(r: UpperBoundedRange<T>) -> Self {
        match r.end.to_bound() {
            Bound::Excluded(t) => ..=Step::backward(t, 1),
            Bound::Included(t) => ..=t,
        }
    }
}

impl<T: Copy + Ord> UpperBoundedRange<T> {
    /// Constructs a new `UpperBoundedRange` from an upper bound.
    ///
    /// # Example
    /// ```
    /// use rangetools::{UpperBound, UpperBoundedRange};
    ///
    /// let r = UpperBoundedRange::new(UpperBound::included(10));
    /// assert!(r.contains(5));
    /// ```
    pub fn new(end: UpperBound<T>) -> Self {
        Self { end }
    }

    /// Returns true if the range contains `t`.
    ///
    /// # Example
    /// ```
    /// use rangetools::Rangetools;
    ///
    /// let i = (..5).intersection(..10);
    /// assert!(i.contains(4));
    /// assert!(!i.contains(9));
    /// ```
    pub fn contains(&self, t: T) -> bool {
        match self.end.0 {
            Bound::Excluded(x) => t < x,
            Bound::Included(i) => t <= i,
        }
    }
}
