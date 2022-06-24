use crate::{
    BoundedRange, BoundedSet, LowerBoundedRange, LowerBoundedSet, RangeIntersection, Rangetools,
    UnboundedRange, UnboundedSet, UpperBoundedRange, UpperBoundedSet, EmptyRange,
};

impl<T, R> RangeIntersection<R, BoundedRange<T>> for std::ops::Range<T>
where
    R: Rangetools<Inner = BoundedRange<T>>,
    T: Copy + Ord,
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(self.to_inner(), other)
    }
}

impl<T, R> RangeIntersection<R, BoundedSet<T>> for std::ops::Range<T>
where
    R: Rangetools<Inner = BoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = BoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(other.to_inner(), self)
    }
}

impl<T, R> RangeIntersection<R, LowerBoundedRange<T>> for std::ops::Range<T>
where
    R: Rangetools<Inner = LowerBoundedRange<T>>,
    T: Copy + Ord,
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(self.to_inner(), other)
    }
}

impl<T, R> RangeIntersection<R, LowerBoundedSet<T>> for std::ops::Range<T>
where
    R: Rangetools<Inner = LowerBoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = BoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(other.to_inner(), self)
    }
}

impl<T, R> RangeIntersection<R, UpperBoundedRange<T>> for std::ops::Range<T>
where
    R: Rangetools<Inner = UpperBoundedRange<T>>,
    T: Copy + Ord,
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(self.to_inner(), other)
    }
}

impl<T, R> RangeIntersection<R, UpperBoundedSet<T>> for std::ops::Range<T>
where
    R: Rangetools<Inner = UpperBoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = BoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(other.to_inner(), self)
    }
}

impl<T, R> RangeIntersection<R, UnboundedRange> for std::ops::Range<T>
where
    R: Rangetools<Inner = UnboundedRange>,
    T: Copy + Ord,
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(self.to_inner(), other)
    }
}

impl<T, R> RangeIntersection<R, UnboundedSet<T>> for std::ops::Range<T>
where
    R: Rangetools<Inner = UnboundedSet<T>>,
    T: Copy + Ord,
{
    type Output = BoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(other.to_inner(), self)
    }
}

impl<T, R> RangeIntersection<R, EmptyRange<T>> for std::ops::Range<T>
where
    R: Rangetools<Inner = EmptyRange<T>>,
    T: Copy + Ord,
{
    type Output = EmptyRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        other.to_inner()
    }
}
