use crate::{LowerBound, LowerBoundedRange};

#[test]
fn next() {
    let r1 = LowerBoundedRange::new(LowerBound::excluded(2));
    let mut i1 = r1.into_iter();
    assert_eq!(i1.next(), Some(3));
    assert_eq!(i1.next(), Some(4));
    assert_eq!(i1.next(), Some(5));

    let r2 = LowerBoundedRange::new(LowerBound::included(2));
    let mut i2 = r2.into_iter();
    assert_eq!(i2.next(), Some(2));
    assert_eq!(i2.next(), Some(3));
    assert_eq!(i2.next(), Some(4));
}

#[test]
fn size_hint() {
    let hint = (usize::MAX, None);
    let r1 = LowerBoundedRange::new(LowerBound::excluded(2));
    assert_eq!(r1.into_iter().size_hint(), hint);

    let r2 = LowerBoundedRange::new(LowerBound::included(2));
    assert_eq!(r2.into_iter().size_hint(), hint);
}

#[test]
fn nth() {
    let r1 = LowerBoundedRange::new(LowerBound::excluded(2));
    let mut i1 = r1.into_iter();
    assert_eq!(i1.nth(1), Some(4));
    assert_eq!(i1.nth(2), Some(7));
    assert_eq!(i1.nth(3), Some(11));

    let r2 = LowerBoundedRange::new(LowerBound::included(2));
    let mut i2 = r2.into_iter();
    assert_eq!(i2.nth(1), Some(3));
    assert_eq!(i2.nth(2), Some(6));
    assert_eq!(i2.nth(3), Some(10));
}

#[test]
fn min() {
    let r1 = LowerBoundedRange::new(LowerBound::excluded(2));
    assert_eq!(r1.into_iter().min(), Some(3));

    let r2 = LowerBoundedRange::new(LowerBound::included(2));
    assert_eq!(r2.into_iter().min(), Some(2));
}
