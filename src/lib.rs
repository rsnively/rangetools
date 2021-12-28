#![warn(missing_docs)]

//! Extra methods for the standard library Range types:
//! * [`std::ops::Range`]
//! * [`std::ops::RangeFrom`]
//! * [`std::ops::RangeFull`]
//! * [`std::ops::RangeInclusive`]
//! * [`std::ops::RangeTo`]
//! * [`std::ops::RangeToInclusive`]
//!
//! To extend these types with the methods in this crate, import the
//! [`Rangetools`] trait:
//!
//! ```
//! use rangetools::Rangetools;
//! ```
//!
//! This provides new methods on all of the above types, as well as any
//! types introduced in this crate to manage the outputs of the such methods.
//!
//! ```
//! use rangetools::Rangetools;
//!
//! let i = (0..5).intersection(3..);
//! assert!(i.contains(4));
//!
//! let i2 = (0..5).intersection(5..10);
//! assert!(i2.is_empty());
//! ```
//!
//! Wherever possible (when the result is lower-bounded), the resulting
//! types of these operations implement [`Iterator`] so that more operations
//! can be performed on them.
//!
//! ```
//! use rangetools::Rangetools;
//!
//! let u1 = (1..3).union(5..7);
//! assert_eq!(u1.collect::<Vec<_>>(), vec![1, 2, 5, 6]);
//!
//! let u2 = (1..3).union(10..);
//! assert_eq!(u2.take(5).collect::<Vec<_>>(), vec![1, 2, 10, 11, 12]);
//!```
//!
//! ```ignored
//! let u3 = (..5).union(10..);
//! let v = u3.collect::<Vec<_>>(); // Compiler error! The resulting union does
//!                                 // not have a lower bound.
//! ```

mod bound;
mod bounded_range;
mod bounded_set;
mod implementations;
mod intersection;
mod lower_bounded_range;
mod lower_bounded_set;
mod step;
#[cfg(test)]
mod test;
mod unbounded_range;
mod unbounded_set;
mod union;
mod upper_bounded_range;
mod upper_bounded_set;

pub use self::{
    bound::*, bounded_range::*, bounded_set::*, intersection::*, lower_bounded_range::*,
    lower_bounded_set::*, step::*, unbounded_range::*, unbounded_set::*, union::*,
    upper_bounded_range::*, upper_bounded_set::*,
};

/// Extends the standard library Range types with extra functionality.
///
/// You shouldn't need to implement this trait yourself. It is already implemented for all of the
/// applicable types from the standard library and all of the intermediate types in this crate.
pub trait Rangetools {
    /// Returns true if the range is empty.
    ///
    /// # Example
    /// ```
    /// use rangetools::Rangetools;
    ///
    /// assert!((3..2).is_empty());
    /// assert!(!(0..5).is_empty());
    /// ```
    fn is_empty(&self) -> bool;

    /// The generalized type of the range (for consolidating range types that only differ in
    /// whether their bounds are inclusive or exclusive).
    ///
    /// In this crate, this will be one of [`BoundedRange`], [`LowerBoundedRange`],
    /// [`UpperBoundedRange`], or [`UnboundedRange`] for the range types, while the set types
    /// all use themselves as an inner type.
    type Inner;
    /// Convert the range to its inner type.
    ///
    /// # Example
    /// ```
    /// use rangetools::{BoundedRange, LowerBound, Rangetools, UpperBound};
    ///
    /// let r = (0..5).to_inner();
    /// assert_eq!(r, BoundedRange { start: LowerBound::included(0), end: UpperBound::excluded(5)});
    /// ```
    fn to_inner(self) -> Self::Inner;

    /// Performs set intersection on `self` and `other`.
    ///
    /// Returns a set/range containing all values contained in both sets/ranges.
    ///
    /// # Example
    /// ```
    /// use rangetools::{BoundedRange, LowerBound, Rangetools, UpperBound};
    ///
    /// let i = (0..5).intersection(3..7);
    /// assert_eq!(i, BoundedRange { start: LowerBound::included(3), end: UpperBound::excluded(5)});
    /// ```
    fn intersection<R, Output>(self, other: R) -> Output
    where
        R: Rangetools,
        Self: Sized + RangeIntersection<R, R::Inner, Output = Output>,
    {
        RangeIntersection::intersection(self, other)
    }

    /// Returns true if two sets/ranges are disjoint (they have no elements in common).
    ///
    /// # Example
    /// ```
    /// use rangetools::Rangetools;
    ///
    /// assert!((0..5).disjoint(10..20));
    /// assert!(!(..5).disjoint(3..));
    /// ```
    fn disjoint<R, Output>(self, other: R) -> bool
    where
        R: Rangetools,
        Output: Rangetools,
        Self: Sized + RangeIntersection<R, R::Inner, Output = Output>,
    {
        Rangetools::intersection(self, other).is_empty()
    }

    /// Returns true if two sets/ranges intersect (they have at least one element in common).
    ///
    /// # Example
    /// ```
    /// use rangetools::Rangetools;
    ///
    /// assert!((..5).intersects(3..));
    /// assert!(!(0..5).intersects(10..20));
    /// ```
    fn intersects<R, Output>(self, other: R) -> bool
    where
        R: Rangetools,
        Output: Rangetools,
        Self: Sized + RangeIntersection<R, R::Inner, Output = Output>,
    {
        !self.disjoint(other)
    }

    /// The set type of the range, for talking about non-contiguous collections of
    /// elements.
    ///
    /// In this crate, this will be one of [`BoundedSet`], [`LowerBoundedSet`],
    /// [`UpperBoundedSet`], or [`UnboundedSet`].
    type Set;
    /// Convert the range to its set type.
    ///
    /// # Example
    /// ```
    /// use rangetools::{BoundedSet, Rangetools};
    ///
    /// let s: BoundedSet<_> = (0..5).to_set();
    /// ```
    fn to_set(self) -> Self::Set;

    /// Performs set union on `self` and `other`.
    ///
    /// Returns a set/range containing all values contained in either set/range.
    ///
    /// # Example
    /// ```
    /// use rangetools::Rangetools;
    ///
    /// let u = (..3).union(5..);
    /// assert!(u.contains(0));
    /// assert!(!u.contains(4));
    /// assert!(u.contains(5));
    /// ```
    fn union<R, Output>(self, other: R) -> Output
    where
        R: Rangetools,
        Self: Sized + RangeUnion<R, R::Set, Output = Output>,
    {
        RangeUnion::union(self, other)
    }
}
