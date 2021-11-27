use crate::{
    BoundedRange, FiniteBound, LowerBoundedRange, RangeIntersection, Rangetools, UnboundedRange,
    UpperBoundedRange,
};

impl<T, R> RangeIntersection<R, BoundedRange<T>> for BoundedRange<T>
where
    T: Copy + Ord,
    R: Rangetools<T, Inner = BoundedRange<T>>,
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        let r = other.to_inner();
        BoundedRange::new(
            FiniteBound::max_start(self.start_bound(), r.start_bound()),
            FiniteBound::min_end(self.end_bound(), r.end_bound()),
        )
    }
}

impl<T, R> RangeIntersection<R, LowerBoundedRange<T>> for BoundedRange<T>
where
    T: Copy + Ord,
    R: Rangetools<T, Inner = LowerBoundedRange<T>>,
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        BoundedRange::new(
            FiniteBound::max_start(self.start_bound(), other.to_inner().start_bound()),
            self.end_bound(),
        )
    }
}

impl<T, R> RangeIntersection<R, UpperBoundedRange<T>> for BoundedRange<T>
where
    T: Copy + Ord,
    R: Rangetools<T, Inner = UpperBoundedRange<T>>,
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        BoundedRange::new(
            self.start_bound(),
            FiniteBound::min_end(self.end_bound(), other.to_inner().end_bound()),
        )
    }
}

impl<T, R> RangeIntersection<R, UnboundedRange<T>> for BoundedRange<T>
where
    R: Rangetools<T, Inner = UnboundedRange<T>>,
{
    type Output = BoundedRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        other.to_inner().intersection(self)
    }
}
