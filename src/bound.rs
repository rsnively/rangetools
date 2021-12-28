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

/// Lower bound of a range.
///
/// Specialization of [`Bound`] where comparisons make sense in all contexts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LowerBound<T>(pub(crate) Bound<T>);

impl<T> From<Bound<T>> for LowerBound<T> {
    fn from(b: Bound<T>) -> Self {
        Self(b)
    }
}

impl<T: PartialOrd> PartialOrd for LowerBound<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (&self.0, &other.0) {
            (Bound::Included(a), Bound::Included(b)) | (Bound::Excluded(a), Bound::Excluded(b)) => {
                a.partial_cmp(&b)
            }
            (Bound::Included(a), Bound::Excluded(b)) => {
                if a == b {
                    Some(std::cmp::Ordering::Less)
                } else {
                    a.partial_cmp(&b)
                }
            }
            (Bound::Excluded(a), Bound::Included(b)) => {
                if a == b {
                    Some(std::cmp::Ordering::Greater)
                } else {
                    a.partial_cmp(&b)
                }
            }
        }
    }
}

impl<T: Ord> Ord for LowerBound<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T> LowerBound<T> {
    /// Constructs a new excluded lower bound.
    ///
    /// # Example
    /// ```
    /// use rangetools::LowerBound;
    ///
    /// let b1 = LowerBound::excluded(2);
    /// let b2 = LowerBound::excluded(3);
    /// assert!(b1 < b2);
    /// ```
    pub fn excluded(t: T) -> Self {
        Self(Bound::Excluded(t))
    }

    /// Constructs a new included lower bound.
    ///
    /// # Example
    /// ```
    /// use rangetools::{LowerBound, LowerBoundedRange};
    ///
    /// let b = LowerBound::included(2);
    /// let r= LowerBoundedRange::from(2..);
    /// assert_eq!(b, r.start);
    /// ```
    pub fn included(t: T) -> Self {
        Self(Bound::Included(t))
    }

    /// Maps a `LowerBound<T>` to a `LowerBound<U>` by applying a function to the contained value.
    ///
    /// # Example
    /// ```
    /// use rangetools::LowerBound;
    ///
    /// let b1 = LowerBound::included(3);
    /// let b2 = b1.map(|n| n + 1);
    /// assert_eq!(b2, LowerBound::included(4));
    /// ```
    pub fn map<U, F>(self, f: F) -> LowerBound<U>
    where
        F: FnOnce(T) -> U,
    {
        LowerBound(self.0.map(f))
    }
}

/// Upper bound of a range.
///
/// Specialization of [`Bound`] where comparisons make sense in all contexts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UpperBound<T>(pub(crate) Bound<T>);

impl<T> From<Bound<T>> for UpperBound<T> {
    fn from(b: Bound<T>) -> Self {
        Self(b)
    }
}

impl<T: PartialOrd> PartialOrd for UpperBound<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (&self.0, &other.0) {
            (Bound::Included(a), Bound::Included(b)) | (Bound::Excluded(a), Bound::Excluded(b)) => {
                a.partial_cmp(&b)
            }
            (Bound::Included(a), Bound::Excluded(b)) => {
                if a == b {
                    Some(std::cmp::Ordering::Greater)
                } else {
                    a.partial_cmp(&b)
                }
            }
            (Bound::Excluded(a), Bound::Included(b)) => {
                if a == b {
                    Some(std::cmp::Ordering::Less)
                } else {
                    a.partial_cmp(&b)
                }
            }
        }
    }
}

impl<T: Ord> Ord for UpperBound<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T> UpperBound<T> {
    /// Constructs a new excluded upper bound.
    ///
    /// # Example
    /// ```
    /// use rangetools::{UpperBound, UpperBoundedRange};
    ///
    /// let b = UpperBound::excluded(2);
    /// let r = UpperBoundedRange::from(..2);
    /// assert_eq!(b, r.end);
    /// ```
    pub fn excluded(t: T) -> Self {
        Self(Bound::Excluded(t))
    }

    /// Constructs a new included upper bound.
    ///
    /// # Example
    /// ```
    /// use rangetools::{UpperBound, UpperBoundedRange};
    ///
    /// let b = UpperBound::included(2);
    /// let r = UpperBoundedRange::from(..=2);
    /// assert_eq!(b, r.end);
    /// ```
    pub fn included(t: T) -> Self {
        Self(Bound::Included(t))
    }

    /// Maps an `UpperBound<T>` to an `UpperBound<U>` by applying a function to the contained value.
    ///
    /// # Example
    /// ```
    /// use rangetools::UpperBound;
    ///
    /// let b1 = UpperBound::included(3);
    /// let b2 = b1.map(|n| n + 1);
    /// assert_eq!(b2, UpperBound::included(4));
    /// ```
    pub fn map<U, F>(self, f: F) -> UpperBound<U>
    where
        F: FnOnce(T) -> U,
    {
        UpperBound(self.0.map(f))
    }
}
