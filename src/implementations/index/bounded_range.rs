use crate::BoundedRange;

impl std::ops::Index<BoundedRange<usize>> for String {
    type Output = str;
    fn index(&self, r: BoundedRange<usize>) -> &Self::Output {
        &self[std::ops::Range::from(r)]
    }
}

impl std::ops::IndexMut<BoundedRange<usize>> for String {
    fn index_mut(&mut self, r: BoundedRange<usize>) -> &mut Self::Output {
        &mut self[std::ops::Range::from(r)]
    }
}

impl std::ops::Index<BoundedRange<usize>> for str {
    type Output = str;
    fn index(&self, r: BoundedRange<usize>) -> &Self::Output {
        &self[std::ops::Range::from(r)]
    }
}

impl std::ops::IndexMut<BoundedRange<usize>> for str {
    fn index_mut(&mut self, r: BoundedRange<usize>) -> &mut Self::Output {
        &mut self[std::ops::Range::from(r)]
    }
}

impl<T> std::ops::Index<BoundedRange<usize>> for [T] {
    type Output = [T];
    fn index(&self, r: BoundedRange<usize>) -> &Self::Output {
        &self[std::ops::Range::from(r)]
    }
}

impl<T> std::ops::IndexMut<BoundedRange<usize>> for [T] {
    fn index_mut(&mut self, r: BoundedRange<usize>) -> &mut Self::Output {
        &mut self[std::ops::Range::from(r)]
    }
}

impl<T> std::ops::Index<BoundedRange<usize>> for Vec<T> {
    type Output = [T];
    fn index(&self, r: BoundedRange<usize>) -> &Self::Output {
        &self[std::ops::Range::from(r)]
    }
}

impl<T> std::ops::IndexMut<BoundedRange<usize>> for Vec<T> {
    fn index_mut(&mut self, r: BoundedRange<usize>) -> &mut Self::Output {
        &mut self[std::ops::Range::from(r)]
    }
}
