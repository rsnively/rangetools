use crate::{
    BoundedRange, BoundedSet, LowerBoundedRange, LowerBoundedSet, PiecewiseUnboundedSet,
    RangeIntersection, RangeUnion, Rangetools, UnboundedRange, UnboundedSet, UpperBoundedRange,
    UpperBoundedSet,
};

impl<T, R> RangeIntersection<R, BoundedRange<T>> for UnboundedSet<T>
where
    R: Rangetools<Inner = BoundedRange<T>>,
    T: Copy + Ord,
{
    type Output = BoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(other.to_inner().to_set(), self)
    }
}

impl<T, R> RangeIntersection<R, BoundedSet<T>> for UnboundedSet<T>
where
    R: Rangetools<Inner = BoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = BoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(other.to_inner(), self)
    }
}

impl<T, R> RangeIntersection<R, LowerBoundedRange<T>> for UnboundedSet<T>
where
    R: Rangetools<Inner = LowerBoundedRange<T>>,
    T: Copy + Ord,
{
    type Output = LowerBoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(other.to_inner().to_set(), self)
    }
}

impl<T, R> RangeIntersection<R, LowerBoundedSet<T>> for UnboundedSet<T>
where
    R: Rangetools<Inner = LowerBoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = LowerBoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(other.to_inner(), self)
    }
}

impl<T, R> RangeIntersection<R, UpperBoundedRange<T>> for UnboundedSet<T>
where
    R: Rangetools<Inner = UpperBoundedRange<T>>,
    T: Copy + Ord,
{
    type Output = UpperBoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(other.to_inner().to_set(), self)
    }
}

impl<T, R> RangeIntersection<R, UpperBoundedSet<T>> for UnboundedSet<T>
where
    R: Rangetools<Inner = UpperBoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = UpperBoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(other.to_inner(), self)
    }
}

impl<T, R> RangeIntersection<R, UnboundedRange> for UnboundedSet<T>
where
    R: Rangetools<Inner = UnboundedRange>,
{
    type Output = UnboundedSet<T>;
    fn intersection(self, _: R) -> Self::Output {
        self
    }
}

impl<T, R> RangeIntersection<R, UnboundedSet<T>> for UnboundedSet<T>
where
    R: Rangetools<Inner = UnboundedSet<T>>,
    T: Copy + Ord,
{
    type Output = UnboundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        match (self, other.to_inner()) {
            (UnboundedSet::Full, other) => other,
            (s, UnboundedSet::Full) => s,
            (
                s,
                UnboundedSet::Piecewise(PiecewiseUnboundedSet {
                    upper_bounded_range: u2,
                    ranges: r2,
                    lower_bounded_range: l2,
                }),
            ) => {
                let a = RangeIntersection::intersection(s.clone(), u2);
                let b = RangeIntersection::intersection(s.clone(), l2);
                let c = RangeIntersection::intersection(s, r2);
                RangeUnion::union(a, RangeUnion::union(b, c))
            }
        }
    }
}
