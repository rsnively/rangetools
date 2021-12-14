use crate::{
    BoundedRange, BoundedSet, LowerBoundedRange, LowerBoundedSet, RangeIntersection, Rangetools,
    UnboundedRange, UnboundedSet, UpperBoundedRange, UpperBoundedSet,
};

impl<T, R> RangeIntersection<R, BoundedRange<T>> for BoundedRange<T>
where
    R: Rangetools<Inner = BoundedRange<T>>,
    T: Copy + Ord,
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        let other = other.to_inner();
        BoundedRange::new(self.start.max(other.start), self.end.min(other.end))
    }
}

impl<T, R> RangeIntersection<R, BoundedSet<T>> for BoundedRange<T>
where
    R: Rangetools<Inner = BoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = BoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(other.to_inner(), self)
    }
}

impl<T, R> RangeIntersection<R, LowerBoundedRange<T>> for BoundedRange<T>
where
    R: Rangetools<Inner = LowerBoundedRange<T>>,
    T: Copy + Ord,
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        BoundedRange::new(self.start.max(other.to_inner().start), self.end)
    }
}

impl<T, R> RangeIntersection<R, LowerBoundedSet<T>> for BoundedRange<T>
where
    R: Rangetools<Inner = LowerBoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = BoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(other.to_inner(), self)
    }
}

impl<T, R> RangeIntersection<R, UpperBoundedRange<T>> for BoundedRange<T>
where
    R: Rangetools<Inner = UpperBoundedRange<T>>,
    T: Copy + Ord,
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        BoundedRange::new(self.start, self.end.min(other.to_inner().end))
    }
}

impl<T, R> RangeIntersection<R, UpperBoundedSet<T>> for BoundedRange<T>
where
    R: Rangetools<Inner = UpperBoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = BoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(other.to_inner(), self)
    }
}

impl<T, R> RangeIntersection<R, UnboundedRange> for BoundedRange<T>
where
    R: Rangetools<Inner = UnboundedRange>,
    T: Copy + Ord,
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(other.to_inner(), self)
    }
}

impl<T, R> RangeIntersection<R, UnboundedSet<T>> for BoundedRange<T>
where
    R: Rangetools<Inner = UnboundedSet<T>>,
    T: Copy + Ord,
{
    type Output = BoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(other.to_inner(), self)
    }
}
