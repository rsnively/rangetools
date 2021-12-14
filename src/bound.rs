/// Endpoints of a closed range.
///
/// A replacement for [`std::ops::Bound`], which can be unwieldy with copy types. More significantly,
/// it codifies the closed nature of the bounds in the type system for tracking whether a range is
/// iterable or not.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Bound<T> {
    Excluded(T),
    Included(T),
}

impl<T: PartialOrd> PartialOrd for Bound<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Included(a), Self::Included(b)) | (Self::Excluded(a), Self::Excluded(b)) => {
                a.partial_cmp(b)
            }
            (Self::Included(a), Self::Excluded(b)) => {
                if a == b {
                    Some(std::cmp::Ordering::Greater)
                } else {
                    a.partial_cmp(b)
                }
            }
            (Self::Excluded(a), Self::Included(b)) => {
                if a == b {
                    Some(std::cmp::Ordering::Less)
                } else {
                    a.partial_cmp(b)
                }
            }
        }
    }
}

impl<T: Ord> Ord for Bound<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T> Bound<T> {
    /// Maps a `Bound<T>` to a `Bound<U>` by applying a function to the contained value.
    ///
    /// # Example
    /// ```
    /// use rangetools::Bound;
    ///
    /// let b1 = Bound::Included(3);
    /// let b2 = b1.map(|n| n + 1);
    /// assert_eq!(b2, Bound::Included(4));
    /// ```
    pub fn map<U, F>(self, f: F) -> Bound<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Excluded(t) => Bound::Excluded(f(t)),
            Self::Included(t) => Bound::Included(f(t)),
        }
    }
}
