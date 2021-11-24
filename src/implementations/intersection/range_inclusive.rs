use crate::{
    BoundedRange, FiniteBound, LowerBoundedRange, RangeIntersection, UnboundedRange,
    UpperBoundedRange,
};

impl<T: Copy + Ord> RangeIntersection<T, std::ops::Range<T>> for std::ops::RangeInclusive<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, other: std::ops::Range<T>) -> Self::Output {
        let r1: BoundedRange<T> = self.into();
        let r2: BoundedRange<T> = other.into();
        BoundedRange::new(
            FiniteBound::max_start(r1.start_bound(), r2.start_bound()),
            FiniteBound::min_end(r1.end_bound(), r2.end_bound()),
        )
    }
}

impl<T: Copy + Ord> RangeIntersection<T, std::ops::RangeFrom<T>> for std::ops::RangeInclusive<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, other: std::ops::RangeFrom<T>) -> Self::Output {
        let r: BoundedRange<T> = self.into();
        let r2: LowerBoundedRange<T> = other.into();
        BoundedRange::new(
            FiniteBound::max_start(r.start_bound(), r2.start_bound()),
            r.end_bound(),
        )
    }
}

impl<T> RangeIntersection<T, std::ops::RangeFull> for std::ops::RangeInclusive<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, _: std::ops::RangeFull) -> Self::Output {
        self.into()
    }
}

impl<T: Copy + Ord> RangeIntersection<T, std::ops::RangeInclusive<T>>
    for std::ops::RangeInclusive<T>
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: std::ops::RangeInclusive<T>) -> Self::Output {
        let r1: BoundedRange<T> = self.into();
        let r2: BoundedRange<T> = other.into();
        BoundedRange::new(
            FiniteBound::max_start(r1.start_bound(), r2.start_bound()),
            FiniteBound::min_end(r1.end_bound(), r2.end_bound()),
        )
    }
}

impl<T: Copy + Ord> RangeIntersection<T, std::ops::RangeTo<T>> for std::ops::RangeInclusive<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, other: std::ops::RangeTo<T>) -> Self::Output {
        let r: BoundedRange<T> = self.into();
        let r2: UpperBoundedRange<T> = other.into();
        BoundedRange::new(
            r.start_bound(),
            FiniteBound::min_end(r.end_bound(), r2.end_bound()),
        )
    }
}

impl<T: Copy + Ord> RangeIntersection<T, std::ops::RangeToInclusive<T>>
    for std::ops::RangeInclusive<T>
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: std::ops::RangeToInclusive<T>) -> Self::Output {
        let r: BoundedRange<T> = self.into();
        let r2: UpperBoundedRange<T> = other.into();
        BoundedRange::new(
            r.start_bound(),
            FiniteBound::min_end(r.end_bound(), r2.end_bound()),
        )
    }
}

impl<T: Copy + Ord> RangeIntersection<T, BoundedRange<T>> for std::ops::RangeInclusive<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, other: BoundedRange<T>) -> Self::Output {
        let r: BoundedRange<T> = self.into();
        BoundedRange::new(
            FiniteBound::max_start(r.start_bound(), other.start_bound()),
            FiniteBound::min_end(r.end_bound(), other.end_bound()),
        )
    }
}

impl<T: Copy + Ord> RangeIntersection<T, LowerBoundedRange<T>> for std::ops::RangeInclusive<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, other: LowerBoundedRange<T>) -> Self::Output {
        let r: BoundedRange<T> = self.into();
        BoundedRange::new(
            FiniteBound::max_start(r.start_bound(), other.start_bound()),
            r.end_bound(),
        )
    }
}

impl<T: Copy + Ord> RangeIntersection<T, UpperBoundedRange<T>> for std::ops::RangeInclusive<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, other: UpperBoundedRange<T>) -> Self::Output {
        let r: BoundedRange<T> = self.into();
        BoundedRange::new(
            r.start_bound(),
            FiniteBound::min_end(r.end_bound(), other.end_bound()),
        )
    }
}

impl<T> RangeIntersection<T, UnboundedRange<T>> for std::ops::RangeInclusive<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, _: UnboundedRange<T>) -> Self::Output {
        self.into()
    }
}
