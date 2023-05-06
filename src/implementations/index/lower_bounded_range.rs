use crate::LowerBoundedRange;

impl std::ops::Index<LowerBoundedRange<usize>> for String {
    type Output = str;
    fn index(&self, r: LowerBoundedRange<usize>) -> &Self::Output {
        &self[std::ops::RangeFrom::from(r)]
    }
}

impl std::ops::IndexMut<LowerBoundedRange<usize>> for String {
    fn index_mut(&mut self, r: LowerBoundedRange<usize>) -> &mut Self::Output {
        &mut self[std::ops::RangeFrom::from(r)]
    }
}

impl std::ops::Index<LowerBoundedRange<usize>> for str {
    type Output = str;
    fn index(&self, r: LowerBoundedRange<usize>) -> &Self::Output {
        &self[std::ops::RangeFrom::from(r)]
    }
}

impl std::ops::IndexMut<LowerBoundedRange<usize>> for str {
    fn index_mut(&mut self, r: LowerBoundedRange<usize>) -> &mut Self::Output {
        &mut self[std::ops::RangeFrom::from(r)]
    }
}

impl std::ops::Index<LowerBoundedRange<usize>> for std::ffi::CStr {
    type Output = std::ffi::CStr;
    fn index(&self, r: LowerBoundedRange<usize>) -> &Self::Output {
        &self[std::ops::RangeFrom::from(r)]
    }
}

impl<T> std::ops::Index<LowerBoundedRange<usize>> for [T] {
    type Output = [T];
    fn index(&self, r: LowerBoundedRange<usize>) -> &Self::Output {
        &self[std::ops::RangeFrom::from(r)]
    }
}

impl<T> std::ops::IndexMut<LowerBoundedRange<usize>> for [T] {
    fn index_mut(&mut self, r: LowerBoundedRange<usize>) -> &mut Self::Output {
        &mut self[std::ops::RangeFrom::from(r)]
    }
}

impl<T> std::ops::Index<LowerBoundedRange<usize>> for Vec<T> {
    type Output = [T];
    fn index(&self, r: LowerBoundedRange<usize>) -> &Self::Output {
        &self[std::ops::RangeFrom::from(r)]
    }
}

impl<T> std::ops::IndexMut<LowerBoundedRange<usize>> for Vec<T> {
    fn index_mut(&mut self, r: LowerBoundedRange<usize>) -> &mut Self::Output {
        &mut self[std::ops::RangeFrom::from(r)]
    }
}
