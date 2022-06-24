use crate::{EmptyRange, RangeUnion, Rangetools};

impl<T, Rhs, RhsSet> RangeUnion<Rhs, RhsSet> for EmptyRange<T>
where
    Rhs: Rangetools<Set = RhsSet>,
{
    type Output = Rhs;
    fn union(self, other: Rhs) -> Self::Output {
        other
    }
}
