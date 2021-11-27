use crate::{
    BoundedRange, LowerBoundedRange, RangeIntersection, Rangetools, UnboundedRange,
    UpperBoundedRange,
};

impl<T, R> RangeIntersection<R, BoundedRange<T>> for std::ops::Range<T>
where
    T: Copy + Ord,
    R: Rangetools<T, Inner = BoundedRange<T>>,
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(self.to_inner(), other)
    }
}

impl<T, R> RangeIntersection<R, LowerBoundedRange<T>> for std::ops::Range<T>
where
    T: Copy + Ord,
    R: Rangetools<T, Inner = LowerBoundedRange<T>>,
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(self.to_inner(), other)
    }
}

impl<T, R> RangeIntersection<R, UpperBoundedRange<T>> for std::ops::Range<T>
where
    T: Copy + Ord,
    R: Rangetools<T, Inner = UpperBoundedRange<T>>,
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(self.to_inner(), other)
    }
}

impl<T, R> RangeIntersection<R, UnboundedRange<T>> for std::ops::Range<T>
where
    R: Rangetools<T, Inner = UnboundedRange<T>>,
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(self.to_inner(), other)
    }
}
