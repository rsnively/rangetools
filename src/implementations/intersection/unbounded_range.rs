use crate::{
    BoundedRange, LowerBoundedRange, RangeIntersection, UnboundedRange, UpperBoundedRange,
};

impl<T> RangeIntersection<T, std::ops::Range<T>> for UnboundedRange<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, other: std::ops::Range<T>) -> Self::Output {
        other.into()
    }
}

impl<T> RangeIntersection<T, std::ops::RangeFrom<T>> for UnboundedRange<T> {
    type Output = LowerBoundedRange<T>;
    fn intersection(self, other: std::ops::RangeFrom<T>) -> Self::Output {
        other.into()
    }
}

impl<T> RangeIntersection<T, std::ops::RangeFull> for UnboundedRange<T> {
    type Output = UnboundedRange<T>;
    fn intersection(self, other: std::ops::RangeFull) -> Self::Output {
        other.into()
    }
}

impl<T> RangeIntersection<T, std::ops::RangeInclusive<T>> for UnboundedRange<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, other: std::ops::RangeInclusive<T>) -> Self::Output {
        other.into()
    }
}

impl<T> RangeIntersection<T, std::ops::RangeTo<T>> for UnboundedRange<T> {
    type Output = UpperBoundedRange<T>;
    fn intersection(self, other: std::ops::RangeTo<T>) -> Self::Output {
        other.into()
    }
}

impl<T> RangeIntersection<T, std::ops::RangeToInclusive<T>> for UnboundedRange<T> {
    type Output = UpperBoundedRange<T>;
    fn intersection(self, other: std::ops::RangeToInclusive<T>) -> Self::Output {
        other.into()
    }
}

impl<T> RangeIntersection<T, BoundedRange<T>> for UnboundedRange<T> {
    type Output = BoundedRange<T>;
    fn intersection(self, other: BoundedRange<T>) -> Self::Output {
        other
    }
}

impl<T> RangeIntersection<T, LowerBoundedRange<T>> for UnboundedRange<T> {
    type Output = LowerBoundedRange<T>;
    fn intersection(self, other: LowerBoundedRange<T>) -> Self::Output {
        other
    }
}

impl<T> RangeIntersection<T, UpperBoundedRange<T>> for UnboundedRange<T> {
    type Output = UpperBoundedRange<T>;
    fn intersection(self, other: UpperBoundedRange<T>) -> Self::Output {
        other
    }
}

impl<T> RangeIntersection<T, UnboundedRange<T>> for UnboundedRange<T> {
    type Output = UnboundedRange<T>;
    fn intersection(self, other: UnboundedRange<T>) -> Self::Output {
        other
    }
}
