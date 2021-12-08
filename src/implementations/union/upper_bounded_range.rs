use crate::{
    BoundedSet, LowerBoundedSet, RangeUnion, Rangetools, UnboundedRange, UnboundedSet,
    UpperBoundedRange, UpperBoundedSet,
};

impl<T, R> RangeUnion<R, BoundedSet<T>> for UpperBoundedRange<T>
where
    R: Rangetools<Set = BoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = UpperBoundedSet<T>;
    fn union(self, other: R) -> Self::Output {
        RangeUnion::union(self.to_set(), other)
    }
}

impl<T, R> RangeUnion<R, LowerBoundedSet<T>> for UpperBoundedRange<T>
where
    R: Rangetools<Set = LowerBoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = UnboundedSet<T>;
    fn union(self, other: R) -> Self::Output {
        RangeUnion::union(self.to_set(), other)
    }
}

impl<T, R> RangeUnion<R, UpperBoundedSet<T>> for UpperBoundedRange<T>
where
    R: Rangetools<Set = UpperBoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = UpperBoundedSet<T>;
    fn union(self, other: R) -> Self::Output {
        RangeUnion::union(self.to_set(), other)
    }
}

impl<T, R> RangeUnion<R, UnboundedSet<T>> for UpperBoundedRange<T>
where
    R: Rangetools<Set = UnboundedSet<T>>,
    T: Copy + Ord,
{
    type Output = UnboundedSet<T>;
    fn union(self, other: R) -> Self::Output {
        RangeUnion::union(self.to_set(), other)
    }
}

impl<T, R> RangeUnion<R, UnboundedRange> for UpperBoundedRange<T>
where
    R: Rangetools<Set = UnboundedRange>,
    T: Copy + Ord,
{
    type Output = UnboundedRange;
    fn union(self, other: R) -> Self::Output {
        RangeUnion::union(other.to_set(), self)
    }
}
