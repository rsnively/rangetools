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

impl<T: Copy + std::ops::AddAssign<T>> std::ops::AddAssign<T> for Bound<T> {
    fn add_assign(&mut self, rhs: T) {
        match self {
            Self::Excluded(t) | Self::Included(t) => *t += rhs,
        }
    }
}

impl<T: Copy> Bound<T> {
    pub fn t(&self) -> T {
        match self {
            Self::Included(t) => *t,
            Self::Excluded(t) => *t,
        }
    }
}
