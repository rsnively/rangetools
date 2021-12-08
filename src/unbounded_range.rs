#[derive(Clone, Copy, Debug)]
pub struct UnboundedRange {}

impl From<std::ops::RangeFull> for UnboundedRange {
    fn from(_: std::ops::RangeFull) -> Self {
        Self {}
    }
}

impl UnboundedRange {
    pub fn contains<T>(&self, _: T) -> bool {
        true
    }

    pub fn is_empty(&self) -> bool {
        false
    }
}
