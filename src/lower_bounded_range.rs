use crate::Bound;

#[derive(Clone, Copy, Debug)]
pub struct LowerBoundedRange<T> {
    pub(crate) start: Bound<T>,
}

impl<T> From<std::ops::RangeFrom<T>> for LowerBoundedRange<T> {
    fn from(r: std::ops::RangeFrom<T>) -> Self {
        Self {
            start: Bound::Included(r.start),
        }
    }
}

impl<T: Copy + Ord> LowerBoundedRange<T> {
    pub fn new(start: Bound<T>) -> Self {
        Self { start }
    }

    pub fn is_empty(&self) -> bool {
        false
    }

    pub fn contains(&self, t: T) -> bool {
        t >= self.start.t()
    }

    pub fn start_bound(&self) -> Bound<T> {
        self.start
    }
}

impl<T> Iterator for LowerBoundedRange<T>
where
    T: Copy + Ord + std::ops::AddAssign<T> + num_traits::One,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.start {
            Bound::Excluded(_) => {
                self.start += T::one();
                Some(self.start.t())
            }
            Bound::Included(t) => {
                self.start += T::one();
                Some(t)
            }
        }
    }
}
