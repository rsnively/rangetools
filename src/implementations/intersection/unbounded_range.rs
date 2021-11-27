use crate::{RangeIntersection, Rangetools, UnboundedRange};

impl<T, B, R: Rangetools<T, Inner = B>> RangeIntersection<R, B> for UnboundedRange<T> {
    type Output = B;
    fn intersection(self, other: R) -> Self::Output {
        other.to_inner()
    }
}
