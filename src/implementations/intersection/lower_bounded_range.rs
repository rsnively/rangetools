use crate::{
    BoundedRange, FiniteBound, LowerBoundedRange, RangeIntersection, Rangetools, UnboundedRange,
    UpperBoundedRange,
};

impl<T, R> RangeIntersection<R, BoundedRange<T>> for LowerBoundedRange<T>
where
    T: Copy + Ord,
    R: Rangetools<T, Inner = BoundedRange<T>>,
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(other.to_inner(), self)
    }
}

impl<T, R> RangeIntersection<R, LowerBoundedRange<T>> for LowerBoundedRange<T>
where
    T: Copy + Ord,
    R: Rangetools<T, Inner = LowerBoundedRange<T>>,
{
    type Output = LowerBoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        LowerBoundedRange::new(FiniteBound::max_start(
            self.start_bound(),
            other.to_inner().start_bound(),
        ))
    }
}

impl<T, R> RangeIntersection<R, UpperBoundedRange<T>> for LowerBoundedRange<T>
where
    T: Copy + Ord,
    R: Rangetools<T, Inner = UpperBoundedRange<T>>,
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        BoundedRange::new(self.start_bound(), other.to_inner().end_bound())
    }
}

impl<T, R> RangeIntersection<R, UnboundedRange<T>> for LowerBoundedRange<T>
where
    R: Rangetools<T, Inner = UnboundedRange<T>>,
{
    type Output = LowerBoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(other.to_inner(), self)
    }
}
