use crate::{
    BoundedSet, LowerBoundedSet, RangeUnion, Rangetools, UnboundedRange, UnboundedSet,
    UpperBoundedSet,
};

impl<T, R> RangeUnion<R, BoundedSet<T>> for BoundedSet<T>
where
    R: Rangetools<Set = BoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = BoundedSet<T>;
    fn union(mut self, other: R) -> Self::Output {
        self.add_set(other.to_set());
        self
    }
}

impl<T, R> RangeUnion<R, LowerBoundedSet<T>> for BoundedSet<T>
where
    R: Rangetools<Set = LowerBoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = LowerBoundedSet<T>;
    fn union(self, other: R) -> Self::Output {
        let mut set = other.to_set();
        set.add_set(self);
        set
    }
}

impl<T, R> RangeUnion<R, UpperBoundedSet<T>> for BoundedSet<T>
where
    R: Rangetools<Set = UpperBoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = UpperBoundedSet<T>;
    fn union(self, other: R) -> Self::Output {
        let mut set = other.to_set();
        set.add_set(self);
        set
    }
}

impl<T, R> RangeUnion<R, UnboundedSet<T>> for BoundedSet<T>
where
    R: Rangetools<Set = UnboundedSet<T>>,
    T: Copy + Ord,
{
    type Output = UnboundedSet<T>;
    fn union(self, other: R) -> Self::Output {
        let mut set = other.to_set();
        set.add_set(self);
        set
    }
}

impl<T, R> RangeUnion<R, UnboundedRange> for BoundedSet<T>
where
    R: Rangetools<Set = UnboundedRange>,
    T: Copy + Ord,
{
    type Output = UnboundedRange;
    fn union(self, other: R) -> Self::Output {
        RangeUnion::union(other.to_set(), self)
    }
}
