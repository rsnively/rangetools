use crate::{
    Bound, BoundedRange, BoundedSet, LowerBoundedRange, LowerBoundedSet, RangeIntersection,
    Rangetools, UnboundedRange, UnboundedSet, UpperBoundedRange, UpperBoundedSet,
};

impl<T, R> RangeIntersection<R, BoundedRange<T>> for BoundedRange<T>
where
    R: Rangetools<Inner = BoundedRange<T>>,
    T: Copy + Ord,
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        let r = other.to_inner();
        BoundedRange::new(
            Bound::max_start(self.start_bound(), r.start_bound()),
            Bound::min_end(self.end_bound(), r.end_bound()),
        )
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
        BoundedRange::new(
            Bound::max_start(self.start_bound(), other.to_inner().start_bound()),
            self.end_bound(),
        )
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
        BoundedRange::new(
            self.start_bound(),
            Bound::min_end(self.end_bound(), other.to_inner().end_bound()),
        )
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
