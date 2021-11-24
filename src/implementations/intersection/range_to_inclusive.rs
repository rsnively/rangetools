use crate::{
    BoundedRange, FiniteBound, LowerBoundedRange, RangeIntersection, UnboundedRange,
    UpperBoundedRange,
};

impl<T: Copy + Ord> RangeIntersection<T, std::ops::Range<T>> for std::ops::RangeToInclusive<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, other: std::ops::Range<T>) -> Self::Output {
        let r1: UpperBoundedRange<T> = self.into();
        let r2: BoundedRange<T> = other.into();
        BoundedRange::new(
            r2.start_bound(),
            FiniteBound::min_end(r1.end_bound(), r2.end_bound()),
        )
    }
}

impl<T: Copy + Ord> RangeIntersection<T, std::ops::RangeFrom<T>> for std::ops::RangeToInclusive<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, other: std::ops::RangeFrom<T>) -> Self::Output {
        BoundedRange::new(
            LowerBoundedRange::from(other).start_bound(),
            UpperBoundedRange::from(self).end_bound(),
        )
    }
}

impl<T> RangeIntersection<T, std::ops::RangeFull> for std::ops::RangeToInclusive<T> {
    type Output = UpperBoundedRange<T>;
    fn intersection(self, _: std::ops::RangeFull) -> Self::Output {
        self.into()
    }
}

impl<T: Copy + Ord> RangeIntersection<T, std::ops::RangeInclusive<T>>
    for std::ops::RangeToInclusive<T>
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: std::ops::RangeInclusive<T>) -> Self::Output {
        let r1: UpperBoundedRange<T> = self.into();
        let r2: BoundedRange<T> = other.into();
        BoundedRange::new(
            r2.start_bound(),
            FiniteBound::min_end(r1.end_bound(), r2.end_bound()),
        )
    }
}

impl<T: Copy + Ord> RangeIntersection<T, std::ops::RangeTo<T>> for std::ops::RangeToInclusive<T> {
    type Output = UpperBoundedRange<T>;
    fn intersection(self, other: std::ops::RangeTo<T>) -> Self::Output {
        UpperBoundedRange::new(FiniteBound::min_end(
            UpperBoundedRange::from(self).end_bound(),
            UpperBoundedRange::from(other).end_bound(),
        ))
    }
}

impl<T: Copy + Ord> RangeIntersection<T, std::ops::RangeToInclusive<T>>
    for std::ops::RangeToInclusive<T>
{
    type Output = UpperBoundedRange<T>;
    fn intersection(self, other: std::ops::RangeToInclusive<T>) -> Self::Output {
        UpperBoundedRange::new(FiniteBound::min_end(
            UpperBoundedRange::from(self).end_bound(),
            UpperBoundedRange::from(other).end_bound(),
        ))
    }
}

impl<T: Copy + Ord> RangeIntersection<T, BoundedRange<T>> for std::ops::RangeToInclusive<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, other: BoundedRange<T>) -> Self::Output {
        BoundedRange::new(
            other.start_bound(),
            FiniteBound::min_end(UpperBoundedRange::from(self).end_bound(), other.end_bound()),
        )
    }
}

impl<T: Copy + Ord> RangeIntersection<T, LowerBoundedRange<T>> for std::ops::RangeToInclusive<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, other: LowerBoundedRange<T>) -> Self::Output {
        BoundedRange::new(
            other.start_bound(),
            UpperBoundedRange::from(self).end_bound(),
        )
    }
}

impl<T: Copy + Ord> RangeIntersection<T, UpperBoundedRange<T>> for std::ops::RangeToInclusive<T> {
    type Output = UpperBoundedRange<T>;
    fn intersection(self, other: UpperBoundedRange<T>) -> Self::Output {
        UpperBoundedRange::new(FiniteBound::min_end(
            UpperBoundedRange::from(self).end_bound(),
            other.end_bound(),
        ))
    }
}

impl<T> RangeIntersection<T, UnboundedRange<T>> for std::ops::RangeToInclusive<T> {
    type Output = UpperBoundedRange<T>;
    fn intersection(self, _: UnboundedRange<T>) -> Self::Output {
        self.into()
    }
}
