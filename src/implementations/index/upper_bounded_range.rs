use crate::UpperBoundedRange;

impl std::ops::Index<UpperBoundedRange<usize>> for String {
    type Output = str;
    fn index(&self, r: UpperBoundedRange<usize>) -> &Self::Output {
        &self[std::ops::RangeTo::from(r)]
    }
}

impl std::ops::IndexMut<UpperBoundedRange<usize>> for String {
    fn index_mut(&mut self, r: UpperBoundedRange<usize>) -> &mut Self::Output {
        &mut self[std::ops::RangeTo::from(r)]
    }
}

impl std::ops::Index<UpperBoundedRange<usize>> for str {
    type Output = str;
    fn index(&self, r: UpperBoundedRange<usize>) -> &Self::Output {
        &self[std::ops::RangeTo::from(r)]
    }
}

impl std::ops::IndexMut<UpperBoundedRange<usize>> for str {
    fn index_mut(&mut self, r: UpperBoundedRange<usize>) -> &mut Self::Output {
        &mut self[std::ops::RangeTo::from(r)]
    }
}

impl<T> std::ops::Index<UpperBoundedRange<usize>> for [T] {
    type Output = [T];
    fn index(&self, r: UpperBoundedRange<usize>) -> &Self::Output {
        &self[std::ops::RangeTo::from(r)]
    }
}

impl<T> std::ops::IndexMut<UpperBoundedRange<usize>> for [T] {
    fn index_mut(&mut self, r: UpperBoundedRange<usize>) -> &mut Self::Output {
        &mut self[std::ops::RangeTo::from(r)]
    }
}

impl<T> std::ops::Index<UpperBoundedRange<usize>> for Vec<T> {
    type Output = [T];
    fn index(&self, r: UpperBoundedRange<usize>) -> &Self::Output {
        &self[std::ops::RangeTo::from(r)]
    }
}

impl<T> std::ops::IndexMut<UpperBoundedRange<usize>> for Vec<T> {
    fn index_mut(&mut self, r: UpperBoundedRange<usize>) -> &mut Self::Output {
        &mut self[std::ops::RangeTo::from(r)]
    }
}
