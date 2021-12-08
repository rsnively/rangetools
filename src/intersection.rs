use crate::Rangetools;

pub trait RangeIntersection<Rhs, RhsInner> {
    type Output;
    fn intersection(self, other: Rhs) -> Self::Output
    where
        Rhs: Rangetools<Inner = RhsInner>;
}
