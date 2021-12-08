use crate::{
    BoundedSet, LowerBoundedSet, RangeUnion, Rangetools, UnboundedRange, UnboundedSet,
    UpperBoundedSet,
};

impl<T, R> RangeUnion<R, BoundedSet<T>> for LowerBoundedSet<T>
where
    R: Rangetools<Set = BoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = LowerBoundedSet<T>;
    fn union(self, other: R) -> Self::Output {
        RangeUnion::union(other.to_set(), self)
    }
}

impl<T, R> RangeUnion<R, LowerBoundedSet<T>> for LowerBoundedSet<T>
where
    R: Rangetools<Set = LowerBoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = LowerBoundedSet<T>;
    fn union(mut self, other: R) -> Self::Output {
        let other = other.to_set();
        self.add_lower_bounded_range(other.lower_bounded_range);
        RangeUnion::union(self, other.ranges)
    }
}

impl<T, R> RangeUnion<R, UpperBoundedSet<T>> for LowerBoundedSet<T>
where
    R: Rangetools<Set = UpperBoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = UnboundedSet<T>;
    fn union(self, other: R) -> Self::Output {
        let other = other.to_set();
        let set = UnboundedSet::new(other.upper_bounded_range, self.lower_bounded_range);
        RangeUnion::union(RangeUnion::union(set, self.ranges), other.ranges)
    }
}

impl<T, R> RangeUnion<R, UnboundedSet<T>> for LowerBoundedSet<T>
where
    R: Rangetools<Set = UnboundedSet<T>>,
    T: Copy + Ord,
{
    type Output = UnboundedSet<T>;
    fn union(self, other: R) -> Self::Output {
        let mut set = other.to_set();
        set.add_lower_bounded_range(self.lower_bounded_range);
        RangeUnion::union(set, self.ranges)
    }
}

impl<T, R> RangeUnion<R, UnboundedRange> for LowerBoundedSet<T>
where
    R: Rangetools<Set = UnboundedRange>,
    T: Copy + Ord,
{
    type Output = UnboundedRange;
    fn union(self, other: R) -> Self::Output {
        RangeUnion::union(other.to_set(), self)
    }
}
