use crate::{
    BoundedRange, FiniteBound, LowerBoundedRange, RangeIntersection, UnboundedRange,
    UpperBoundedRange,
};

impl<T: Copy + Ord> RangeIntersection<T, std::ops::Range<T>> for UpperBoundedRange<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, other: std::ops::Range<T>) -> Self::Output {
        let r: BoundedRange<T> = other.into();
        BoundedRange::new(
            r.start_bound(),
            FiniteBound::min_end(self.end_bound(), r.end_bound()),
        )
    }
}

impl<T: Copy + Ord> RangeIntersection<T, std::ops::RangeFrom<T>> for UpperBoundedRange<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, other: std::ops::RangeFrom<T>) -> Self::Output {
        let r: LowerBoundedRange<T> = other.into();
        BoundedRange::new(r.start_bound(), self.end_bound())
    }
}

impl<T> RangeIntersection<T, std::ops::RangeFull> for UpperBoundedRange<T> {
    type Output = UpperBoundedRange<T>;
    fn intersection(self, _: std::ops::RangeFull) -> Self::Output {
        self
    }
}

impl<T: Copy + Ord> RangeIntersection<T, std::ops::RangeInclusive<T>> for UpperBoundedRange<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, other: std::ops::RangeInclusive<T>) -> Self::Output {
        let r: BoundedRange<T> = other.into();
        BoundedRange::new(
            r.start_bound(),
            FiniteBound::min_end(self.end_bound(), r.end_bound()),
        )
    }
}

impl<T: Copy + Ord> RangeIntersection<T, std::ops::RangeTo<T>> for UpperBoundedRange<T> {
    type Output = UpperBoundedRange<T>;
    fn intersection(self, other: std::ops::RangeTo<T>) -> Self::Output {
        let r: UpperBoundedRange<T> = other.into();
        UpperBoundedRange::new(FiniteBound::min_end(self.end_bound(), r.end_bound()))
    }
}

impl<T: Copy + Ord> RangeIntersection<T, std::ops::RangeToInclusive<T>> for UpperBoundedRange<T> {
    type Output = UpperBoundedRange<T>;
    fn intersection(self, other: std::ops::RangeToInclusive<T>) -> Self::Output {
        let r: UpperBoundedRange<T> = other.into();
        UpperBoundedRange::new(FiniteBound::min_end(self.end_bound(), r.end_bound()))
    }
}

impl<T: Copy + Ord> RangeIntersection<T, BoundedRange<T>> for UpperBoundedRange<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, other: BoundedRange<T>) -> Self::Output {
        BoundedRange::new(
            other.start_bound(),
            FiniteBound::min_end(self.end_bound(), other.end_bound()),
        )
    }
}

impl<T: Copy + Ord> RangeIntersection<T, LowerBoundedRange<T>> for UpperBoundedRange<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, other: LowerBoundedRange<T>) -> Self::Output {
        BoundedRange::new(other.start_bound(), self.end_bound())
    }
}

impl<T: Copy + Ord> RangeIntersection<T, UpperBoundedRange<T>> for UpperBoundedRange<T> {
    type Output = UpperBoundedRange<T>;
    fn intersection(self, other: UpperBoundedRange<T>) -> Self::Output {
        UpperBoundedRange::new(FiniteBound::min_end(self.end_bound(), other.end_bound()))
    }
}

impl<T> RangeIntersection<T, UnboundedRange<T>> for UpperBoundedRange<T> {
    type Output = UpperBoundedRange<T>;
    fn intersection(self, _: UnboundedRange<T>) -> Self::Output {
        self
    }
}
