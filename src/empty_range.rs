#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// A range with no elements.
///
/// The `std::ops` ranges have the concept of "empty" ranges, but there's no really good
/// way of constructing them without providing garbage values to them. Constructing ranges
/// with indices that don't make any sense are "empty"
/// ```
/// assert!((4..2).is_empty());
/// ```
/// but we'd like to provide an "empty" range type for operations that'll come up
/// when doing Rangetools calculations.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EmptyRange<T> {
    t: PhantomData<T>,
}

impl<T> IntoIterator for EmptyRange<T> {
    type IntoIter = std::iter::Empty<T>;
    type Item = T;
    fn into_iter(self) -> Self::IntoIter {
        std::iter::empty()
    }
}

impl<T> EmptyRange<T> {
    /// Constructs a new `EmptyRange`.
    ///
    /// # Example
    /// ```
    /// use rangetools::{EmptyRange, Rangetools};
    ///
    /// let r = EmptyRange::<i32>::new();
    /// assert!(r.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            t: PhantomData::default(),
        }
    }

    /// Returns true if the range contains the given item.
    ///
    /// This function always returns false, but it's nice to have it for instances
    /// when you don't want to have to think about what type your range operation
    /// is returning.
    ///
    /// # Example
    /// ```
    /// use rangetools::EmptyRange;
    ///
    /// let r = EmptyRange::new();
    /// assert!(!r.contains(5));
    /// ```
    pub fn contains(&self, _: T) -> bool {
        false
    }
}
