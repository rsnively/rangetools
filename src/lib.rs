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
#[cfg(test)]
mod test;
mod unbounded_range;
mod unbounded_set;
mod union;
mod upper_bounded_range;
mod upper_bounded_set;

pub use self::{
    bound::*, bounded_range::*, bounded_set::*, intersection::*, lower_bounded_range::*,
    lower_bounded_set::*, unbounded_range::*, unbounded_set::*, union::*, upper_bounded_range::*,
    upper_bounded_set::*,
};

pub trait Rangetools {
    fn is_empty(&self) -> bool;

    type Inner;
    fn to_inner(self) -> Self::Inner;
    fn intersection<R, Output>(self, other: R) -> Output
    where
        R: Rangetools,
        Self: Sized + RangeIntersection<R, R::Inner, Output = Output>,
    {
        RangeIntersection::intersection(self, other)
    }

    type Set;
    fn to_set(self) -> Self::Set;
    fn union<R, Output>(self, other: R) -> Output
    where
        R: Rangetools,
        Self: Sized + RangeUnion<R, R::Set, Output = Output>,
    {
        RangeUnion::union(self, other)
    }
}
