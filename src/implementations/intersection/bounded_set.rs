use crate::{
    BoundedRange, BoundedSet, LowerBoundedRange, LowerBoundedSet, PiecewiseUnboundedSet,
    RangeIntersection, Rangetools, UnboundedRange, UnboundedSet, UpperBoundedRange,
    UpperBoundedSet,
};

impl<T, R> RangeIntersection<R, BoundedRange<T>> for BoundedSet<T>
where
    R: Rangetools<Inner = BoundedRange<T>>,
    T: Copy + Ord,
{
    type Output = BoundedSet<T>;
    fn intersection(mut self, other: R) -> Self::Output {
        let other = other.to_inner();
        for range in self.ranges.iter_mut() {
            *range = RangeIntersection::intersection(*range, other);
        }
        self.ranges.retain(|r| !r.is_empty());
        self
    }
}

impl<T, R> RangeIntersection<R, BoundedSet<T>> for BoundedSet<T>
where
    R: Rangetools<Inner = BoundedSet<T>> + Clone,
    T: Copy + Ord,
{
    type Output = BoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        let mut set = BoundedSet::empty();
        for range in self.ranges.iter() {
            set.add_set(RangeIntersection::intersection(*range, other.clone()));
        }
        set
    }
}

impl<T, R> RangeIntersection<R, LowerBoundedRange<T>> for BoundedSet<T>
where
    R: Rangetools<Inner = LowerBoundedRange<T>>,
    T: Copy + Ord,
{
    type Output = BoundedSet<T>;
    fn intersection(mut self, other: R) -> Self::Output {
        let other = other.to_inner();
        for range in self.ranges.iter_mut() {
            *range = RangeIntersection::intersection(*range, other);
        }
        self.ranges.retain(|r| !r.is_empty());
        self
    }
}

impl<T, R> RangeIntersection<R, LowerBoundedSet<T>> for BoundedSet<T>
where
    R: Rangetools<Inner = LowerBoundedSet<T>> + Clone,
    T: Copy + Ord,
{
    type Output = BoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        let mut set = RangeIntersection::intersection(
            self.clone(),
            other.clone().to_inner().lower_bounded_range,
        );
        for range in self.ranges.iter() {
            set.add_set(RangeIntersection::intersection(*range, other.clone()));
        }
        set
    }
}

impl<T, R> RangeIntersection<R, UpperBoundedRange<T>> for BoundedSet<T>
where
    R: Rangetools<Inner = UpperBoundedRange<T>>,
    T: Copy + Ord,
{
    type Output = BoundedSet<T>;
    fn intersection(mut self, other: R) -> Self::Output {
        let other = other.to_inner();
        for range in self.ranges.iter_mut() {
            *range = RangeIntersection::intersection(*range, other);
        }
        self.ranges.retain(|r| !r.is_empty());
        self
    }
}

impl<T, R> RangeIntersection<R, UpperBoundedSet<T>> for BoundedSet<T>
where
    R: Rangetools<Inner = UpperBoundedSet<T>> + Clone,
    T: Copy + Ord,
{
    type Output = BoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        let mut set = RangeIntersection::intersection(
            self.clone(),
            other.clone().to_inner().upper_bounded_range,
        );
        for range in self.ranges.iter() {
            set.add_set(RangeIntersection::intersection(*range, other.clone()));
        }
        set
    }
}

impl<T, R> RangeIntersection<R, UnboundedRange> for BoundedSet<T>
where
    R: Rangetools<Inner = UnboundedRange>,
    T: Copy + Ord,
{
    type Output = BoundedSet<T>;
    fn intersection(self, _: R) -> Self::Output {
        self
    }
}

impl<T, R> RangeIntersection<R, UnboundedSet<T>> for BoundedSet<T>
where
    R: Rangetools<Inner = UnboundedSet<T>>,
    T: Copy + Ord,
{
    type Output = BoundedSet<T>;
    fn intersection(self, other: R) -> Self::Output {
        match other.to_inner() {
            UnboundedSet::Full => self,
            UnboundedSet::Piecewise(PiecewiseUnboundedSet {
                lower_bounded_range,
                ranges,
                upper_bounded_range,
            }) => {
                let mut set = RangeIntersection::intersection(self.clone(), upper_bounded_range);
                set.add_set(RangeIntersection::intersection(
                    self.clone(),
                    lower_bounded_range,
                ));
                set.add_set(RangeIntersection::intersection(self, ranges));
                set
            }
        }
    }
}
