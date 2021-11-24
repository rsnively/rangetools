pub trait RangeIntersection<T, Rhs> {
    type Output;
    fn intersection(self, other: Rhs) -> Self::Output;
}
