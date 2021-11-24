use crate::{
    BoundedRange, LowerBoundedRange, RangeIntersection, UnboundedRange, UpperBoundedRange,
};

impl<T> RangeIntersection<T, std::ops::Range<T>> for std::ops::RangeFull {
    type Output = BoundedRange<T>;
    fn intersection(self, other: std::ops::Range<T>) -> Self::Output {
        other.into()
    }
}

impl<T> RangeIntersection<T, std::ops::RangeFrom<T>> for std::ops::RangeFull {
    type Output = LowerBoundedRange<T>;
    fn intersection(self, other: std::ops::RangeFrom<T>) -> Self::Output {
        other.into()
    }
}

impl<T> RangeIntersection<T, std::ops::RangeFull> for std::ops::RangeFull {
    type Output = UnboundedRange<T>;
    fn intersection(self, _: std::ops::RangeFull) -> Self::Output {
        self.into()
    }
}

impl<T> RangeIntersection<T, std::ops::RangeInclusive<T>> for std::ops::RangeFull {
    type Output = BoundedRange<T>;
    fn intersection(self, other: std::ops::RangeInclusive<T>) -> Self::Output {
        other.into()
    }
}

impl<T> RangeIntersection<T, std::ops::RangeTo<T>> for std::ops::RangeFull {
    type Output = UpperBoundedRange<T>;
    fn intersection(self, other: std::ops::RangeTo<T>) -> Self::Output {
        other.into()
    }
}

impl<T> RangeIntersection<T, std::ops::RangeToInclusive<T>> for std::ops::RangeFull {
    type Output = UpperBoundedRange<T>;
    fn intersection(self, other: std::ops::RangeToInclusive<T>) -> Self::Output {
        other.into()
    }
}

impl<T> RangeIntersection<T, BoundedRange<T>> for std::ops::RangeFull {
    type Output = BoundedRange<T>;
    fn intersection(self, other: BoundedRange<T>) -> Self::Output {
        other
    }
}

impl<T> RangeIntersection<T, LowerBoundedRange<T>> for std::ops::RangeFull {
    type Output = LowerBoundedRange<T>;
    fn intersection(self, other: LowerBoundedRange<T>) -> Self::Output {
        other
    }
}

impl<T> RangeIntersection<T, UpperBoundedRange<T>> for std::ops::RangeFull {
    type Output = UpperBoundedRange<T>;
    fn intersection(self, other: UpperBoundedRange<T>) -> Self::Output {
        other
    }
}

impl<T> RangeIntersection<T, UnboundedRange<T>> for std::ops::RangeFull {
    type Output = UnboundedRange<T>;
    fn intersection(self, other: UnboundedRange<T>) -> Self::Output {
        other
    }
}
