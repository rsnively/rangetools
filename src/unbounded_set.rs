use crate::{
    Bound, BoundedRange, BoundedSet, LowerBoundedRange, RangeIntersection, UnboundedRange,
    UpperBoundedRange,
};

#[derive(Clone, Debug)]
pub struct PiecewiseUnboundedSet<T> {
    pub(crate) upper_bounded_range: UpperBoundedRange<T>,
    pub(crate) ranges: BoundedSet<T>,
    pub(crate) lower_bounded_range: LowerBoundedRange<T>,
}

impl<T: Copy + Ord> PiecewiseUnboundedSet<T> {
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
            .position(|r| r.start_bound() <= self.upper_bounded_range.end_bound())
        {
            let range = self.ranges.ranges.remove(index);
            self.upper_bounded_range.end = self.upper_bounded_range.end.max(range.end_bound());
            self.defragment();
        } else if let Some(index) = self
            .ranges
            .ranges
            .iter()
            .position(|r| r.end_bound() >= self.lower_bounded_range.start_bound())
        {
            let range = self.ranges.ranges.remove(index);
            self.lower_bounded_range.start =
                self.lower_bounded_range.start.min(range.start_bound());
            self.defragment();
        }
    }
}

#[derive(Clone, Debug)]
pub enum UnboundedSet<T> {
    Full,
    Piecewise(PiecewiseUnboundedSet<T>),
}

impl<T> From<UnboundedRange> for UnboundedSet<T> {
    fn from(_: UnboundedRange) -> Self {
        Self::Full
    }
}

impl<T: Copy + Ord> UnboundedSet<T> {
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

impl<T> UnboundedSet<T> {
    pub fn is_empty(&self) -> bool {
        false
    }
}

impl<T: Copy + Ord> UnboundedSet<T> {
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
            if upper_bounded_range.end_bound() >= lower_bounded_range.start_bound() {
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
