use crate::{
    BoundedRange, EmptyRange, LowerBound, LowerBoundedRange, UnboundedRange, UpperBoundedRange,
};

#[test]
fn range() {
    let a: BoundedRange<_> = (0..10).into();
    let b: std::ops::Range<_> = a.into();
    assert_eq!(b, 0..10);

    let c: BoundedRange<_> = (0..=10).into();
    let d: std::ops::Range<_> = c.into();
    assert_eq!(d, 0..11);

    let e: EmptyRange<i32> = EmptyRange::new();
    let d: std::ops::Range<_> = e.into();
    assert!(d.is_empty());
}

#[test]
fn range_inclusive() {
    let a: BoundedRange<_> = (0..10).into();
    let b: std::ops::RangeInclusive<_> = a.into();
    assert_eq!(b, 0..=9);

    let c: BoundedRange<_> = (0..=10).into();
    let d: std::ops::RangeInclusive<_> = c.into();
    assert_eq!(d, 0..=10);

    let e: EmptyRange<i32> = EmptyRange::new();
    let d: std::ops::RangeInclusive<_> = e.into();
    assert!(d.is_empty());
}

#[test]
fn range_to() {
    let a: UpperBoundedRange<_> = (..10).into();
    let b: std::ops::RangeTo<_> = a.into();
    assert_eq!(b, ..10);

    let c: UpperBoundedRange<_> = (..=10).into();
    let d: std::ops::RangeTo<_> = c.into();
    assert_eq!(d, ..11);
}

#[test]
fn range_to_inclusive() {
    let a: UpperBoundedRange<_> = (..10).into();
    let b: std::ops::RangeToInclusive<_> = a.into();
    assert_eq!(b, ..=9);

    let c: UpperBoundedRange<_> = (..=10).into();
    let d: std::ops::RangeToInclusive<_> = c.into();
    assert_eq!(d, ..=10);
}

#[test]
fn range_from() {
    let a: LowerBoundedRange<_> = (10..).into();
    let b: std::ops::RangeFrom<_> = a.into();
    assert_eq!(b, 10..);

    let c = LowerBoundedRange::new(LowerBound::excluded(10));
    let d: std::ops::RangeFrom<_> = c.into();
    assert_eq!(d, 11..);
}

#[test]
fn range_full() {
    let a = UnboundedRange;
    let b: std::ops::RangeFull = a.into();
    assert_eq!(b, ..);
}
