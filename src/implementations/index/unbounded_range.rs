use crate::UnboundedRange;

impl std::ops::Index<UnboundedRange> for String {
    type Output = str;
    fn index(&self, r: UnboundedRange) -> &Self::Output {
        &self[std::ops::RangeFull::from(r)]
    }
}

impl std::ops::IndexMut<UnboundedRange> for String {
    fn index_mut(&mut self, r: UnboundedRange) -> &mut Self::Output {
        &mut self[std::ops::RangeFull::from(r)]
    }
}

impl std::ops::Index<UnboundedRange> for str {
    type Output = str;
    fn index(&self, r: UnboundedRange) -> &Self::Output {
        &self[std::ops::RangeFull::from(r)]
    }
}

impl std::ops::IndexMut<UnboundedRange> for str {
    fn index_mut(&mut self, r: UnboundedRange) -> &mut Self::Output {
        &mut self[std::ops::RangeFull::from(r)]
    }
}

impl std::ops::Index<UnboundedRange> for std::ffi::CString {
    type Output = std::ffi::CStr;
    fn index(&self, r: UnboundedRange) -> &Self::Output {
        &self[std::ops::RangeFull::from(r)]
    }
}

impl std::ops::Index<UnboundedRange> for std::ffi::OsString {
    type Output = std::ffi::OsStr;
    fn index(&self, r: UnboundedRange) -> &Self::Output {
        &self[std::ops::RangeFull::from(r)]
    }
}

impl std::ops::IndexMut<UnboundedRange> for std::ffi::OsString {
    fn index_mut(&mut self, r: UnboundedRange) -> &mut Self::Output {
        &mut self[std::ops::RangeFull::from(r)]
    }
}

impl<T> std::ops::Index<UnboundedRange> for [T] {
    type Output = [T];
    fn index(&self, r: UnboundedRange) -> &Self::Output {
        &self[std::ops::RangeFull::from(r)]
    }
}

impl<T> std::ops::IndexMut<UnboundedRange> for [T] {
    fn index_mut(&mut self, r: UnboundedRange) -> &mut Self::Output {
        &mut self[std::ops::RangeFull::from(r)]
    }
}

impl<T> std::ops::Index<UnboundedRange> for Vec<T> {
    type Output = [T];
    fn index(&self, r: UnboundedRange) -> &Self::Output {
        &self[std::ops::RangeFull::from(r)]
    }
}

impl<T> std::ops::IndexMut<UnboundedRange> for Vec<T> {
    fn index_mut(&mut self, r: UnboundedRange) -> &mut Self::Output {
        &mut self[std::ops::RangeFull::from(r)]
    }
}
