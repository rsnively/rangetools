use crate::{Bound, LowerBound, Rangetools, Step, UpperBound};

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

impl<T> Iterator for BoundedRange<T>
where
    T: Copy + Ord + Step,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_empty() {
            None
        } else {
            match (self.start.0, self.end.0) {
                (Bound::Included(start), _) => {
                    self.start = self.start.map(T::next);
                    Some(start)
                }
                (Bound::Excluded(start), Bound::Included(_)) => {
                    self.start = self.start.map(T::next);
                    Some(start.next())
                }
                (Bound::Excluded(start), Bound::Excluded(end)) => {
                    let next = start.next();
                    if next >= end {
                        None
                    } else {
                        self.start = self.start.map(T::next);
                        Some(next)
                    }
                }
            }
        }
    }
}
