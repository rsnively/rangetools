/// A range with no upper or lower bound.
///
/// Generalizes over [`std::ops::RangeFull`].
///
/// While an `UnboundedRange` can be constructed directly, it will most likely
/// result from one or more range operations.
/// ```
/// use rangetools::{UnboundedRange, Rangetools};
///
/// let i = (..).intersection(..);
/// assert_eq!(i, UnboundedRange {});
/// ```
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UnboundedRange {}

impl From<std::ops::RangeFull> for UnboundedRange {
    fn from(_: std::ops::RangeFull) -> Self {
        Self {}
    }
}

impl UnboundedRange {
    /// Constructs a new `UnboundedRange`.
    ///
    /// # Example
    /// ```
    /// use rangetools::UnboundedRange;
    ///
    /// let r = UnboundedRange::new();
    /// assert!(r.contains(42));
    /// ```
    pub fn new() -> Self {
        Self {}
    }

    /// Returns true if the range contains the given item.
    ///
    /// This function always returns true, but it's nice to have it for instances
    /// when you don't want to have to think about what type your range operation
    /// is returning.
    ///
    /// # Example
    /// ```
    /// use rangetools::Rangetools;
    ///
    /// let r = (..).intersection(..);
    /// assert!(r.contains(5));
    /// assert!(r.contains(3));
    /// ```
    pub fn contains<T>(&self, _: T) -> bool {
        true
    }
}
