use crate::Rangetools;

pub trait RangeUnion<Rhs, RhsSet> {
    type Output;
    fn union(self, other: Rhs) -> Self::Output
    where
        Rhs: Rangetools<Set = RhsSet>;
}
