use crate::{
    BoundedRange, BoundedSet, EmptyRange, LowerBoundedRange, LowerBoundedSet,
    PiecewiseUnboundedSet, RangeIntersection, Rangetools, UnboundedRange, UnboundedSet,
    UpperBoundedRange, UpperBoundedSet,
};

impl<T, R> RangeIntersection<R, BoundedRange<T>> for LowerBoundedSet<T>
where
    R: Rangetools<Inner = BoundedRange<T>>,
    T: Copy + Ord,
{
    type Output = BoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        let other = other.to_inner();
        let mut set: BoundedSet<T> =
            RangeIntersection::intersection(self.lower_bounded_range, other).into();
        set.add_set(RangeIntersection::intersection(self.ranges, other));
        set
    }
}

impl<T, R> RangeIntersection<R, BoundedSet<T>> for LowerBoundedSet<T>
where
    R: Rangetools<Inner = BoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = BoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        RangeIntersection::intersection(other.to_inner(), self)
    }
}

impl<T, R> RangeIntersection<R, LowerBoundedRange<T>> for LowerBoundedSet<T>
where
    R: Rangetools<Inner = LowerBoundedRange<T>>,
    T: Copy + Ord,
{
    type Output = LowerBoundedSet<T>;
    fn intersection(mut self, other: R) -> Self::Output {
        let other = other.to_inner();
        self.lower_bounded_range = RangeIntersection::intersection(self.lower_bounded_range, other);
        self.ranges = RangeIntersection::intersection(self.ranges, other);
        self
    }
}

impl<T, R> RangeIntersection<R, LowerBoundedSet<T>> for LowerBoundedSet<T>
where
    R: Rangetools<Inner = LowerBoundedSet<T>> + Clone,
    T: Copy + Ord,
{
    type Output = LowerBoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        let mut set = RangeIntersection::intersection(
            self.clone(),
            other.clone().to_inner().lower_bounded_range,
        );
        set.add_set(RangeIntersection::intersection(self.ranges, other));
        set
    }
}

impl<T, R> RangeIntersection<R, UpperBoundedRange<T>> for LowerBoundedSet<T>
where
    R: Rangetools<Inner = UpperBoundedRange<T>>,
    T: Copy + Ord,
{
    type Output = BoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        let other = other.to_inner();
        let mut set = RangeIntersection::intersection(self.lower_bounded_range, other).to_set();
        set.ranges.retain(|r| !r.is_empty());
        set.add_set(RangeIntersection::intersection(self.ranges, other));
        set.ranges.retain(|r| !r.is_empty());
        set
    }
}

impl<T, R> RangeIntersection<R, UpperBoundedSet<T>> for LowerBoundedSet<T>
where
    R: Rangetools<Inner = UpperBoundedSet<T>>,
    T: Copy + Ord,
{
    type Output = BoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        let other = other.to_inner();
        let mut set = RangeIntersection::intersection(self.clone(), other.upper_bounded_range);
        set.ranges.retain(|r| !r.is_empty());
        set.add_set(RangeIntersection::intersection(self, other.ranges));
        set.ranges.retain(|r| !r.is_empty());
        set
    }
}

impl<T, R> RangeIntersection<R, UnboundedRange> for LowerBoundedSet<T>
where
    R: Rangetools<Inner = UnboundedRange>,
    T: Copy + Ord,
{
    type Output = LowerBoundedSet<T>;
    fn intersection(self, _: R) -> Self::Output {
        self
    }
}

impl<T, R> RangeIntersection<R, UnboundedSet<T>> for LowerBoundedSet<T>
where
    R: Rangetools<Inner = UnboundedSet<T>> + Clone,
    T: Copy + Ord,
{
    type Output = LowerBoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        match other.to_inner() {
            UnboundedSet::Full => self,
            UnboundedSet::Piecewise(PiecewiseUnboundedSet {
                lower_bounded_range,
                ranges,
                upper_bounded_range,
            }) => {
                let mut set = RangeIntersection::intersection(self.clone(), lower_bounded_range);
                set.add_set(RangeIntersection::intersection(
                    self.clone(),
                    upper_bounded_range,
                ));
                set.add_set(RangeIntersection::intersection(self, ranges));
                set
            }
        }
    }
}

impl<T, R> RangeIntersection<R, EmptyRange<T>> for LowerBoundedSet<T>
where
    R: Rangetools<Inner = EmptyRange<T>>,
    T: Copy + Ord,
{
    type Output = EmptyRange<T>;
    fn intersection(self, other: R) -> Self::Output {
        other.to_inner()
    }
}
