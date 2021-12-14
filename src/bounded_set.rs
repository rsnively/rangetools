use crate::{BoundedRange, Rangetools, Step};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct BoundedSet<T> {
    pub(crate) ranges: Vec<BoundedRange<T>>,
}

impl<T: Copy + Ord> From<BoundedRange<T>> for BoundedSet<T> {
    fn from(r: BoundedRange<T>) -> Self {
        if r.is_empty() {
            Self::empty()
        } else {
            Self { ranges: vec![r] }
        }
    }
}

impl<T: Copy + Ord + Step> Iterator for BoundedSet<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.ranges.first_mut().map(|r| r.next()).flatten();
        self.ranges.retain(|r| !r.is_empty());
        ret
    }
}

impl<T> BoundedSet<T> {
    pub fn empty() -> Self {
        Self { ranges: Vec::new() }
    }
}

impl<T: Copy + Ord> BoundedSet<T> {
    pub(crate) fn add_range(&mut self, r: BoundedRange<T>) {
        if !r.is_empty() {
            if let Some(index) = self.ranges.iter().position(|range| !range.disjoint(r)) {
                let new_range = r.combine(&self.ranges[index]);
                self.ranges.remove(index);
                self.add_range(new_range);
            } else {
                let index = self
                    .ranges
                    .iter()
                    .position(|range| range.start_bound() > r.end_bound())
                    .unwrap_or(self.ranges.len());
                self.ranges.insert(index, r);
            }
        }
    }
    pub(crate) fn add_set(&mut self, other: Self) {
        for range in other.ranges.into_iter() {
            self.add_range(range);
        }
    }

    pub fn contains(&self, t: T) -> bool {
        self.ranges.iter().any(|r| r.contains(t))
    }
}
