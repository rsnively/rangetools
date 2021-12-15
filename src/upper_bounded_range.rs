use crate::Bound;

/// A range only bounded above (either inclusive or exclusive).
///
/// Generalizes over [`std::ops::RangeTo`] and [`std::ops::RangeToInclusive`].
///
/// While an `UpperoundedRange` can be constructed directly, it will most likely
/// result from one or more range operations.
/// ```
/// use rangetools::{Bound, UpperBoundedRange, Rangetools};
///
/// let i = (..5).intersection(..=3);
/// assert_eq!(i, UpperBoundedRange { end: Bound::Included(3) });
/// ```
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct UpperBoundedRange<T> {
    /// The upper bound of the range (can be inclusive or exclusive).
    pub end: Bound<T>,
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
    /// Constructs a new `UpperBoundedRange` from an upper bound.
    ///
    /// # Example
    /// ```
    /// use rangetools::{Bound, UpperBoundedRange};
    ///
    /// let r = UpperBoundedRange::new(Bound::Included(10));
    /// assert!(r.contains(5));
    /// ```
    pub fn new(end: Bound<T>) -> Self {
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
        match self.end {
            Bound::Excluded(x) => t < x,
            Bound::Included(i) => t <= i,
        }
    }
}
