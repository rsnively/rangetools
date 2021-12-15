use crate::{
    Bound, BoundedRange, BoundedSet, LowerBoundedRange, RangeIntersection, Rangetools,
    UnboundedRange, UpperBoundedRange,
};

/// A set of ranges with ultimately no upper or lower bound.
///
/// A piecewise unbounded set may contain gaps in an otherwise full set.
/// # Example
/// ```
/// use rangetools::Rangetools;
///
/// let piecewise = (..3).union(5..10).union(20..);
/// assert!(piecewise.contains(0));
/// assert!(!piecewise.contains(4));
/// assert!(piecewise.contains(7));
/// assert!(!piecewise.contains(15));
/// assert!(piecewise.contains(42));
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct PiecewiseUnboundedSet<T> {
    /// Kept private to enforce the invariant that the ranges be non-empty and non-overlapping.
    pub(crate) upper_bounded_range: UpperBoundedRange<T>,
    pub(crate) ranges: BoundedSet<T>,
    pub(crate) lower_bounded_range: LowerBoundedRange<T>,
}

impl<T: Copy + Ord> PiecewiseUnboundedSet<T> {
    /// Returns true if the set contains `t`.
    ///
    /// # Example
    /// ```
    /// use rangetools::Rangetools;
    ///
    /// let s = (..5).union(10..);
    /// assert!(s.contains(4));
    /// assert!(!s.contains(7));
    /// assert!(s.contains(10));
    /// ```
    pub fn contains(&self, t: T) -> bool {
        self.upper_bounded_range.contains(t)
            || self.lower_bounded_range.contains(t)
            || self.ranges.contains(t)
    }

    fn defragment(&mut self) {
        if let Some(index) = self
            .ranges
            .ranges
            .iter()
            .position(|r| r.start <= self.upper_bounded_range.end)
        {
            let range = self.ranges.ranges.remove(index);
            self.upper_bounded_range.end = self.upper_bounded_range.end.max(range.end);
            self.defragment();
        } else if let Some(index) = self
            .ranges
            .ranges
            .iter()
            .position(|r| r.end >= self.lower_bounded_range.start)
        {
            let range = self.ranges.ranges.remove(index);
            self.lower_bounded_range.start = self.lower_bounded_range.start.min(range.start);
            self.defragment();
        }
    }
}

/// A set of ranges ultimately with no upper or lower bound.
///
/// An `UnboundedSet` can be constructed directly, but it will most often arise as a
/// result of one or more range operations.
/// ```
/// use rangetools::{UnboundedSet, Rangetools};
///
/// let piecewise: UnboundedSet<_> = (..3).union(5..);
/// let full: UnboundedSet<_> = (..10).union(5..);
/// assert_eq!(full, UnboundedSet::Full);
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum UnboundedSet<T> {
    // Denotes an `UnboundedSet` containing all possible values of T.
    Full,
    // Denotes an `UnboundedSet` which may contain ranges of non-contained values.
    Piecewise(PiecewiseUnboundedSet<T>),
}

impl<T> From<UnboundedRange> for UnboundedSet<T> {
    fn from(_: UnboundedRange) -> Self {
        Self::Full
    }
}

impl<T: Copy + Ord> UnboundedSet<T> {
    /// Construct an `UnboundedSet` from an `UpperBoundedRange` and a `LowerBoundedRange`.
    ///
    /// # Example
    /// ```
    /// use rangetools::{LowerBoundedRange, UnboundedSet, UpperBoundedRange};
    ///
    /// let s = UnboundedSet::new(UpperBoundedRange::from(..5), LowerBoundedRange::from(10..));
    /// ```
    pub fn new(
        upper_bounded_range: UpperBoundedRange<T>,
        lower_bounded_range: LowerBoundedRange<T>,
    ) -> Self {
        if RangeIntersection::intersection(upper_bounded_range, lower_bounded_range).is_empty() {
            Self::Piecewise(PiecewiseUnboundedSet {
                upper_bounded_range,
                ranges: BoundedSet::empty(),
                lower_bounded_range,
            })
        } else {
            Self::Full
        }
    }
}

impl<T: Copy + Ord> UnboundedSet<T> {
    /// Returns true if the set contains `t`.
    ///
    /// # Example
    /// ```
    /// use rangetools::UnboundedSet;
    ///
    /// let s = UnboundedSet::Full;
    /// assert!(s.contains(4));
    /// assert!(s.contains(7));
    /// assert!(s.contains(10));
    /// ```
    pub fn contains(&self, t: T) -> bool {
        match self {
            Self::Full => true,
            Self::Piecewise(p) => p.contains(t),
        }
    }
    fn map_piecewise(&mut self, f: impl FnOnce(&mut PiecewiseUnboundedSet<T>)) {
        match self {
            Self::Full => {}
            Self::Piecewise(p) => f(p),
        }
    }
    fn defragment(&mut self) {
        self.map_piecewise(PiecewiseUnboundedSet::defragment);
        if let Self::Piecewise(PiecewiseUnboundedSet {
            upper_bounded_range,
            lower_bounded_range,
            ..
        }) = self
        {
            if upper_bounded_range.end >= lower_bounded_range.start {
                *self = Self::Full
            }
        }
    }

    pub(crate) fn add_range(&mut self, range: BoundedRange<T>) {
        self.map_piecewise(|p| p.ranges.add_range(range));
        self.defragment();
    }
    pub(crate) fn add_lower_bounded_range(&mut self, range: LowerBoundedRange<T>) {
        self.map_piecewise(|p| {
            p.lower_bounded_range.start = Bound::min(p.lower_bounded_range.start, range.start);
        });
        self.defragment();
    }
    pub(crate) fn add_upper_bounded_range(&mut self, range: UpperBoundedRange<T>) {
        self.map_piecewise(|p| {
            p.upper_bounded_range.end = Bound::max(p.upper_bounded_range.end, range.end);
        });
        self.defragment();
    }
    pub(crate) fn add_set(&mut self, set: BoundedSet<T>) {
        for range in set.ranges.into_iter() {
            self.add_range(range);
        }
    }
    pub(crate) fn add_unbounded_set(&mut self, range: UnboundedSet<T>) {
        match range {
            Self::Piecewise(PiecewiseUnboundedSet {
                lower_bounded_range,
                ranges,
                upper_bounded_range,
            }) => {
                self.add_lower_bounded_range(lower_bounded_range);
                self.add_upper_bounded_range(upper_bounded_range);
                self.add_set(ranges);
                self.defragment();
            }
            Self::Full => *self = Self::Full,
        }
    }
}
