use crate::{Bound, LowerBound, Step};

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

impl<T> Iterator for LowerBoundedRange<T>
where
    T: Copy + Ord + Step,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.start.0 {
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
