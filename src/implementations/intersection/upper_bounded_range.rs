use crate::{
    BoundedRange, BoundedSet, LowerBoundedRange, LowerBoundedSet, RangeIntersection, Rangetools,
    UnboundedRange, UnboundedSet, UpperBoundedRange, UpperBoundedSet,
};

impl<T, R> RangeIntersection<R, BoundedRange<T>> for UpperBoundedRange<T>
where
    R: Rangetools<Inner = BoundedRange<T>>,
    T: Copy + Ord,
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(other.to_inner(), self)
    }
}

impl<T, R> RangeIntersection<R, BoundedSet<T>> for UpperBoundedRange<T>
where
    R: Rangetools<Inner = BoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = BoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(other.to_inner(), self)
    }
}

impl<T, R> RangeIntersection<R, LowerBoundedRange<T>> for UpperBoundedRange<T>
where
    R: Rangetools<Inner = LowerBoundedRange<T>>,
    T: Copy + Ord,
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(other.to_inner(), self)
    }
}

impl<T, R> RangeIntersection<R, LowerBoundedSet<T>> for UpperBoundedRange<T>
where
    R: Rangetools<Inner = LowerBoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = BoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(other.to_inner(), self)
    }
}

impl<T, R> RangeIntersection<R, UpperBoundedRange<T>> for UpperBoundedRange<T>
where
    R: Rangetools<Inner = UpperBoundedRange<T>>,
    T: Copy + Ord,
{
    type Output = UpperBoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        UpperBoundedRange::new(self.end.min(other.to_inner().end))
    }
}

impl<T, R> RangeIntersection<R, UpperBoundedSet<T>> for UpperBoundedRange<T>
where
    R: Rangetools<Inner = UpperBoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = UpperBoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(other.to_inner(), self)
    }
}

impl<T, R> RangeIntersection<R, UnboundedRange> for UpperBoundedRange<T>
where
    R: Rangetools<Inner = UnboundedRange>,
{
    type Output = UpperBoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(other.to_inner(), self)
    }
}

impl<T, R> RangeIntersection<R, UnboundedSet<T>> for UpperBoundedRange<T>
where
    R: Rangetools<Inner = UnboundedSet<T>>,
    T: Copy + Ord,
{
    type Output = UpperBoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(other.to_inner(), self)
    }
}
