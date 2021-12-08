use crate::{RangeUnion, Rangetools, UnboundedRange};

// impl<T, R> RangeUnion<R, BoundedSet<T>> for UnboundedRange<T>
// where
//     R: Rangetools<Set = BoundedSet<T>>,
// {
//     type Output = UnboundedSet<T>;
//     fn union(self, other: R) -> Self::Output {
//         RangeUnion::union(self.to_set(), other)
//     }
// }

// impl<T, R> RangeUnion<R, LowerBoundedSet<T>> for UnboundedRange<T>
// where
//     R: Rangetools<Set = LowerBoundedSet<T>>,
// {
//     type Output = UnboundedSet<T>;
//     fn union(self, other: R) -> Self::Output {
//         RangeUnion::union(self.to_set(), other)
//     }
// }

// impl<T, R> RangeUnion<R, UpperBoundedSet<T>> for UnboundedRange<T>
// where
//     R: Rangetools<Set = UpperBoundedSet<T>>,
// {
//     type Output = UnboundedSet<T>;
//     fn union(self, other: R) -> Self::Output {
//         RangeUnion::union(self.to_set(), other)
//     }
// }

// impl<T, R> RangeUnion<R, UnboundedSet<T>> for UnboundedRange<T>
// where
//     R: Rangetools<Set = UnboundedSet<T>>,
// {
//     type Output = UnboundedSet<T>;
//     fn union(self, other: R) -> Self::Output {
//         RangeUnion::union(self.to_set(), other)
//     }
// }

impl<Set, R: Rangetools<Set = Set>> RangeUnion<R, Set> for UnboundedRange {
    type Output = UnboundedRange;
    fn union(self, _: R) -> Self::Output {
        self
    }
}
