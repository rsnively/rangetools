#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UnboundedRange {}

impl From<std::ops::RangeFull> for UnboundedRange {
    fn from(_: std::ops::RangeFull) -> Self {
        Self {}
    }
}

impl UnboundedRange {
    pub fn new() -> Self {
        Self {}
    }

    pub fn contains<T>(&self, _: T) -> bool {
        true
    }
}
