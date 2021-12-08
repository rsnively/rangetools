use crate::{RangeIntersection, Rangetools, UnboundedRange};

impl<B, R: Rangetools<Inner = B>> RangeIntersection<R, B> for UnboundedRange {
    type Output = B;
    fn intersection(self, other: R) -> Self::Output {
        other.to_inner()
    }
}
