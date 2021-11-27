use crate::{
    BoundedRange, FiniteBound, LowerBoundedRange, RangeIntersection, Rangetools, UnboundedRange,
    UpperBoundedRange,
};

impl<T, R> RangeIntersection<R, BoundedRange<T>> for UpperBoundedRange<T>
where
    T: Copy + Ord,
    R: Rangetools<T, Inner = BoundedRange<T>>,
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        other.to_inner().intersection(self)
    }
}

impl<T, R> RangeIntersection<R, LowerBoundedRange<T>> for UpperBoundedRange<T>
where
    T: Copy + Ord,
    R: Rangetools<T, Inner = LowerBoundedRange<T>>,
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        other.to_inner().intersection(self)
    }
}

impl<T, R> RangeIntersection<R, UpperBoundedRange<T>> for UpperBoundedRange<T>
where
    T: Copy + Ord,
    R: Rangetools<T, Inner = UpperBoundedRange<T>>,
{
    type Output = UpperBoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        UpperBoundedRange::new(FiniteBound::min_end(
            self.end_bound(),
            other.to_inner().end_bound(),
        ))
    }
}

impl<T, R> RangeIntersection<R, UnboundedRange<T>> for UpperBoundedRange<T>
where
    R: Rangetools<T, Inner = UnboundedRange<T>>,
{
    type Output = UpperBoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        other.to_inner().intersection(self)
    }
}
