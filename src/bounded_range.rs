use crate::FiniteBound;

#[derive(Clone, Copy, Debug)]
pub struct BoundedRange<T> {
    start: FiniteBound<T>,
    end: FiniteBound<T>,
}

impl<T> From<std::ops::Range<T>> for BoundedRange<T> {
    fn from(r: std::ops::Range<T>) -> Self {
        Self {
            start: FiniteBound::Included(r.start),
            end: FiniteBound::Excluded(r.end),
        }
    }
}

impl<T> From<std::ops::RangeInclusive<T>> for BoundedRange<T> {
    fn from(r: std::ops::RangeInclusive<T>) -> Self {
        let (start, end) = r.into_inner();
        Self {
            start: FiniteBound::Included(start),
            end: FiniteBound::Included(end),
        }
    }
}

impl<T: Copy + Ord> BoundedRange<T> {
    pub fn new(start: FiniteBound<T>, end: FiniteBound<T>) -> Self {
        Self { start, end }
    }

    pub fn start_bound(&self) -> FiniteBound<T> {
        self.start
    }
    pub fn end_bound(&self) -> FiniteBound<T> {
        self.end
    }

    pub fn is_empty(&self) -> bool {
        self.start_bound() > self.end_bound()
    }

    pub fn contains(&self, t: T) -> bool {
        let start_satisfied = match self.start {
            FiniteBound::Excluded(s) => t > s,
            FiniteBound::Included(s) => t >= s,
        };
        let end_satisfied = match self.end {
            FiniteBound::Excluded(e) => t < e,
            FiniteBound::Included(e) => t <= e,
        };
        start_satisfied && end_satisfied
    }
}

impl<T> Iterator for BoundedRange<T>
where
    T: Copy + Ord + std::ops::AddAssign<T> + std::ops::Add<T, Output = T> + num_traits::One,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match (self.start, self.end) {
            (FiniteBound::Included(start), FiniteBound::Included(end)) => {
                if start > end {
                    None
                } else {
                    self.start += T::one();
                    Some(start)
                }
            }
            (FiniteBound::Included(start), FiniteBound::Excluded(end)) => {
                if start >= end {
                    None
                } else {
                    self.start += T::one();
                    Some(start)
                }
            }
            (FiniteBound::Excluded(start), FiniteBound::Included(end)) => {
                if start >= end {
                    None
                } else {
                    self.start += T::one();
                    Some(self.start.t())
                }
            }
            (FiniteBound::Excluded(start), FiniteBound::Excluded(end)) => {
                if start + T::one() >= end {
                    None
                } else {
                    self.start += T::one();
                    Some(self.start.t())
                }
            }
        }
    }
}
