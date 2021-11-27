mod bound;
mod bounded_range;
mod implementations;
mod intersection;
mod lower_bounded_range;
#[cfg(test)]
mod test;
mod unbounded_range;
mod upper_bounded_range;

pub use self::{
    bound::*, bounded_range::*, intersection::*, lower_bounded_range::*, unbounded_range::*,
    upper_bounded_range::*,
};

pub trait Rangetools<T> {
    type Inner;
    fn to_inner(self) -> Self::Inner;
    fn intersection<R, Output>(self, other: R) -> Output
    where
        R: Rangetools<T>,
        Self: Sized + RangeIntersection<R, R::Inner, Output = Output>,
    {
        RangeIntersection::intersection(self, other)
    }
}

impl<T> Rangetools<T> for std::ops::Range<T> {
    type Inner = BoundedRange<T>;
    fn to_inner(self) -> Self::Inner {
        self.into()
    }
}
impl<T> Rangetools<T> for std::ops::RangeFrom<T> {
    type Inner = LowerBoundedRange<T>;
    fn to_inner(self) -> Self::Inner {
        self.into()
    }
}
impl<T> Rangetools<T> for std::ops::RangeFull {
    type Inner = UnboundedRange<T>;
    fn to_inner(self) -> Self::Inner {
        self.into()
    }
}
impl<T> Rangetools<T> for std::ops::RangeInclusive<T> {
    type Inner = BoundedRange<T>;
    fn to_inner(self) -> Self::Inner {
        self.into()
    }
}
impl<T> Rangetools<T> for std::ops::RangeTo<T> {
    type Inner = UpperBoundedRange<T>;
    fn to_inner(self) -> Self::Inner {
        self.into()
    }
}
impl<T> Rangetools<T> for std::ops::RangeToInclusive<T> {
    type Inner = UpperBoundedRange<T>;
    fn to_inner(self) -> Self::Inner {
        self.into()
    }
}

impl<T> Rangetools<T> for BoundedRange<T> {
    type Inner = Self;
    fn to_inner(self) -> Self::Inner {
        self
    }
}

impl<T> Rangetools<T> for LowerBoundedRange<T> {
    type Inner = Self;
    fn to_inner(self) -> Self::Inner {
        self
    }
}

impl<T> Rangetools<T> for UpperBoundedRange<T> {
    type Inner = Self;
    fn to_inner(self) -> Self::Inner {
        self
    }
}

impl<T> Rangetools<T> for UnboundedRange<T> {
    type Inner = Self;
    fn to_inner(self) -> Self::Inner {
        self
    }
}
