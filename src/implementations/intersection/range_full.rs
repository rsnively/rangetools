use crate::{
    BoundedRange, BoundedSet, LowerBoundedRange, LowerBoundedSet, RangeIntersection, Rangetools,
    UnboundedRange, UnboundedSet, UpperBoundedRange, UpperBoundedSet,
};

impl<T, R> RangeIntersection<R, BoundedRange<T>> for std::ops::RangeFull
where
    R: Rangetools<Inner = BoundedRange<T>>,
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(self.to_inner(), other)
    }
}

impl<T, R> RangeIntersection<R, BoundedSet<T>> for std::ops::RangeFull
where
    R: Rangetools<Inner = BoundedSet<T>>,
{
    type Output = BoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(self.to_inner(), other)
    }
}

impl<T, R> RangeIntersection<R, LowerBoundedRange<T>> for std::ops::RangeFull
where
    R: Rangetools<Inner = LowerBoundedRange<T>>,
{
    type Output = LowerBoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(self.to_inner(), other)
    }
}

impl<T, R> RangeIntersection<R, LowerBoundedSet<T>> for std::ops::RangeFull
where
    R: Rangetools<Inner = LowerBoundedSet<T>>,
{
    type Output = LowerBoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(self.to_inner(), other)
    }
}

impl<T, R> RangeIntersection<R, UpperBoundedRange<T>> for std::ops::RangeFull
where
    R: Rangetools<Inner = UpperBoundedRange<T>>,
{
    type Output = UpperBoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(self.to_inner(), other)
    }
}

impl<T, R> RangeIntersection<R, UpperBoundedSet<T>> for std::ops::RangeFull
where
    R: Rangetools<Inner = UpperBoundedSet<T>>,
{
    type Output = UpperBoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(self.to_inner(), other)
    }
}

impl<R> RangeIntersection<R, UnboundedRange> for std::ops::RangeFull
where
    R: Rangetools<Inner = UnboundedRange>,
{
    type Output = UnboundedRange;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(self.to_inner(), other)
    }
}

impl<T, R> RangeIntersection<R, UnboundedSet<T>> for std::ops::RangeFull
where
    R: Rangetools<Inner = UnboundedSet<T>>,
{
    type Output = UnboundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(self.to_inner(), other)
    }
}
