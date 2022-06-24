use crate::{EmptyRange, RangeIntersection, Rangetools};

impl<T, Rhs, RhsInner> RangeIntersection<Rhs, RhsInner> for EmptyRange<T>
where
    Rhs: Rangetools<Inner = RhsInner>,
{
    type Output = EmptyRange<T>;
    fn intersection(self, _: Rhs) -> Self::Output {
        self
    }
}
