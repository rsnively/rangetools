use crate::{BoundedRange, LowerBound, UpperBound};

#[test]
fn text_next_included_included() {
    let mut r: BoundedRange<i32> = (0..=3).into();
    assert_eq!(r.next(), Some(0));
    assert_eq!(r.next(), Some(1));
    assert_eq!(r.next(), Some(2));
    assert_eq!(r.next(), Some(3));
    assert_eq!(r.next(), None);
}

#[test]
fn test_next_included_excluded() {
    let mut r: BoundedRange<i32> = (0..3).into();
    assert_eq!(r.next(), Some(0));
    assert_eq!(r.next(), Some(1));
    assert_eq!(r.next(), Some(2));
    assert_eq!(r.next(), None);
}

#[test]
fn test_next_excluded_included() {
    let mut r = BoundedRange::new(LowerBound::excluded(3), UpperBound::included(5));
    assert_eq!(r.next(), Some(4));
    assert_eq!(r.next(), Some(5));
    assert_eq!(r.next(), None);
}

#[test]
fn test_next_excluded_excluded() {
    let mut r = BoundedRange::new(LowerBound::excluded(3), UpperBound::excluded(5));
    assert_eq!(r.next(), Some(4));
    assert_eq!(r.next(), None);
}
