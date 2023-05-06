use crate::EmptyRange;

impl std::ops::Index<EmptyRange<usize>> for String {
    type Output = str;
    fn index(&self, r: EmptyRange<usize>) -> &Self::Output {
        &self[std::ops::Range::from(r)]
    }
}

impl std::ops::Index<EmptyRange<usize>> for str {
    type Output = str;
    fn index(&self, r: EmptyRange<usize>) -> &Self::Output {
        &self[std::ops::Range::from(r)]
    }
}

impl<T> std::ops::Index<EmptyRange<usize>> for [T] {
    type Output = [T];
    fn index(&self, r: EmptyRange<usize>) -> &Self::Output {
        &self[std::ops::Range::from(r)]
    }
}

impl<T> std::ops::Index<EmptyRange<usize>> for Vec<T> {
    type Output = [T];
    fn index(&self, r: EmptyRange<usize>) -> &Self::Output {
        &self[std::ops::Range::from(r)]
    }
}
