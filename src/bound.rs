#[derive(Clone, Copy, Debug)]
pub enum Bound<T> {
    Unbounded,
    Bounded(FiniteBound<T>),
}

impl<T: Copy> From<std::ops::Bound<&T>> for Bound<T> {
    fn from(b: std::ops::Bound<&T>) -> Self {
        match b {
            std::ops::Bound::Unbounded => Self::Unbounded,
            std::ops::Bound::Excluded(t) => Self::Bounded(FiniteBound::Excluded(*t)),
            std::ops::Bound::Included(t) => Self::Bounded(FiniteBound::Included(*t)),
        }
    }
}

impl<T: Copy + Ord> Bound<T> {
    pub fn bounded(&self) -> Option<FiniteBound<T>> {
        match self {
            Self::Unbounded => None,
            Self::Bounded(b) => Some(*b),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FiniteBound<T> {
    Excluded(T),
    Included(T),
}

impl<T: PartialOrd> PartialOrd for FiniteBound<T> {
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

impl<T: Ord> Ord for FiniteBound<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T: Copy + std::ops::AddAssign<T>> std::ops::AddAssign<T> for FiniteBound<T> {
    fn add_assign(&mut self, rhs: T) {
        match self {
            Self::Excluded(t) | Self::Included(t) => *t += rhs,
        }
    }
}

impl<T: Copy + Ord> FiniteBound<T> {
    pub fn t(&self) -> T {
        match self {
            Self::Included(t) => *t,
            Self::Excluded(t) => *t,
        }
    }
    pub(crate) fn max_start(a: Self, b: Self) -> Self {
        match (a, b) {
            (Self::Excluded(a), Self::Excluded(b)) => Self::Excluded(a.max(b)),
            (Self::Included(a), Self::Included(b)) => Self::Included(a.max(b)),
            (Self::Included(i), Self::Excluded(x)) | (Self::Excluded(x), Self::Included(i)) => {
                if i > x {
                    Self::Included(i)
                } else {
                    Self::Excluded(x)
                }
            }
        }
    }
    pub(crate) fn min_end(a: Self, b: Self) -> Self {
        match (a, b) {
            (Self::Excluded(a), Self::Excluded(b)) => Self::Excluded(a.min(b)),
            (Self::Included(a), Self::Included(b)) => Self::Included(a.min(b)),
            (Self::Included(i), Self::Excluded(x)) | (Self::Excluded(x), Self::Included(i)) => {
                if i < x {
                    Self::Included(i)
                } else {
                    Self::Excluded(x)
                }
            }
        }
    }
}