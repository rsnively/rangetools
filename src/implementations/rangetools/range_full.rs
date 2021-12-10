use crate::{Rangetools, UnboundedRange};

impl Rangetools for std::ops::RangeFull {
    fn is_empty(&self) -> bool {
        false
    }

    type Inner = UnboundedRange;
    fn to_inner(self) -> Self::Inner {
        self.into()
    }

    type Set = UnboundedRange;
    fn to_set(self) -> Self::Set {
        self.into()
    }
}
