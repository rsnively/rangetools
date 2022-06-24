use crate::{RangeUnion, Rangetools, UnboundedRange};

impl<Set, R: Rangetools<Set = Set>> RangeUnion<R, Set> for UnboundedRange {
    type Output = UnboundedRange;
    fn union(self, _: R) -> Self::Output {
        self
    }
}
