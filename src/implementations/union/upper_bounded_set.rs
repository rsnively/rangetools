use crate::{
    BoundedSet, LowerBoundedSet, RangeUnion, Rangetools, UnboundedRange, UnboundedSet,
    UpperBoundedSet,
};

impl<T, R> RangeUnion<R, BoundedSet<T>> for UpperBoundedSet<T>
where
    R: Rangetools<Set = BoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = UpperBoundedSet<T>;
    fn union(self, other: R) -> Self::Output {
        RangeUnion::union(other.to_set(), self)
    }
}

impl<T, R> RangeUnion<R, LowerBoundedSet<T>> for UpperBoundedSet<T>
where
    R: Rangetools<Set = LowerBoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = UnboundedSet<T>;
    fn union(self, other: R) -> Self::Output {
        RangeUnion::union(other.to_set(), self)
    }
}

impl<T, R> RangeUnion<R, UpperBoundedSet<T>> for UpperBoundedSet<T>
where
    R: Rangetools<Set = UpperBoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = UpperBoundedSet<T>;
    fn union(mut self, other: R) -> Self::Output {
        let other = other.to_set();
        self.add_upper_bounded_range(other.upper_bounded_range);
        RangeUnion::union(self, other.ranges)
    }
}

impl<T, R> RangeUnion<R, UnboundedSet<T>> for UpperBoundedSet<T>
where
    R: Rangetools<Set = UnboundedSet<T>>,
    T: Copy + Ord,
{
    type Output = UnboundedSet<T>;
    fn union(self, other: R) -> Self::Output {
        let mut set = other.to_set();
        set.add_upper_bounded_range(self.upper_bounded_range);
        RangeUnion::union(set, self.ranges)
    }
}

impl<T, R> RangeUnion<R, UnboundedRange> for UpperBoundedSet<T>
where
    R: Rangetools<Set = UnboundedRange>,
    T: Copy + Ord,
{
    type Output = UnboundedRange;
    fn union(self, other: R) -> Self::Output {
        RangeUnion::union(other.to_set(), self)
    }
}
