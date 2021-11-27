use crate::{
    BoundedRange, LowerBoundedRange, RangeIntersection, Rangetools, UnboundedRange,
    UpperBoundedRange,
};

impl<T, R> RangeIntersection<R, BoundedRange<T>> for std::ops::RangeFull
where
    R: Rangetools<T, Inner = BoundedRange<T>>,
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(self.to_inner(), other)
    }
}

impl<T, R> RangeIntersection<R, LowerBoundedRange<T>> for std::ops::RangeFull
where
    R: Rangetools<T, Inner = LowerBoundedRange<T>>,
{
    type Output = LowerBoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(self.to_inner(), other)
    }
}

impl<T, R> RangeIntersection<R, UpperBoundedRange<T>> for std::ops::RangeFull
where
    R: Rangetools<T, Inner = UpperBoundedRange<T>>,
{
    type Output = UpperBoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(self.to_inner(), other)
    }
}

impl<T, R> RangeIntersection<R, UnboundedRange<T>> for std::ops::RangeFull
where
    R: Rangetools<T, Inner = UnboundedRange<T>>,
{
    type Output = UnboundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(self.to_inner(), other)
    }
}
