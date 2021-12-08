use crate::{
    BoundedSet, LowerBoundedSet, RangeUnion, Rangetools, UnboundedRange, UnboundedSet,
    UpperBoundedSet,
};

impl<T, R> RangeUnion<R, BoundedSet<T>> for std::ops::RangeFull
where
    R: Rangetools<Set = BoundedSet<T>>,
{
    type Output = UnboundedRange;
    fn union(self, other: R) -> Self::Output {
        RangeUnion::union(self.to_set(), other)
    }
}

impl<T, R> RangeUnion<R, LowerBoundedSet<T>> for std::ops::RangeFull
where
    R: Rangetools<Set = LowerBoundedSet<T>>,
{
    type Output = UnboundedRange;
    fn union(self, other: R) -> Self::Output {
        RangeUnion::union(self.to_set(), other)
    }
}

impl<T, R> RangeUnion<R, UpperBoundedSet<T>> for std::ops::RangeFull
where
    R: Rangetools<Set = UpperBoundedSet<T>>,
{
    type Output = UnboundedRange;
    fn union(self, other: R) -> Self::Output {
        RangeUnion::union(self.to_set(), other)
    }
}

impl<T, R> RangeUnion<R, UnboundedSet<T>> for std::ops::RangeFull
where
    R: Rangetools<Set = UnboundedSet<T>>,
{
    type Output = UnboundedRange;
    fn union(self, other: R) -> Self::Output {
        RangeUnion::union(self.to_set(), other)
    }
}

impl<R> RangeUnion<R, UnboundedRange> for std::ops::RangeFull
where
    R: Rangetools<Set = UnboundedRange>,
{
    type Output = UnboundedRange;
    fn union(self, other: R) -> Self::Output {
        RangeUnion::union(other.to_set(), self)
    }
}
