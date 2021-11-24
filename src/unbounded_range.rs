use std::marker::PhantomData;

#[derive(Clone, Copy, Debug)]
pub struct UnboundedRange<T> {
    data: PhantomData<T>,
}

impl<T> From<std::ops::RangeFull> for UnboundedRange<T> {
    fn from(_: std::ops::RangeFull) -> Self {
        Self {
            data: Default::default(),
        }
    }
}

impl<T> UnboundedRange<T> {
    pub fn contains(&self, _: T) -> bool {
        true
    }

    pub fn is_empty(&self) -> bool {
        false
    }
}
