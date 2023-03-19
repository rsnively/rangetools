use crate::{BoundedRange, LowerBound, RangeUnion, UpperBound};

#[test]
fn text_next_included_included() {
    let r: BoundedRange<i32> = (0..=3).into();
    let mut it = r.into_iter();
    assert_eq!(it.next(), Some(0));
    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next(), Some(3));
    assert_eq!(it.next(), None);

    for i in (0..3).union(5..10) {
        println!("{i}");
    }
}

#[test]
fn test_next_included_excluded() {
    let r: BoundedRange<i32> = (0..3).into();
    let mut it = r.into_iter();
    assert_eq!(it.next(), Some(0));
    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next(), None);
}

#[test]
fn test_next_excluded_included() {
    let r = BoundedRange::new(LowerBound::excluded(3), UpperBound::included(5));
    let mut it = r.into_iter();
    assert_eq!(it.next(), Some(4));
    assert_eq!(it.next(), Some(5));
    assert_eq!(it.next(), None);
}

#[test]
fn test_next_excluded_excluded() {
    let r = BoundedRange::new(LowerBound::excluded(3), UpperBound::excluded(5));
    let mut it = r.into_iter();
    assert_eq!(it.next(), Some(4));
    assert_eq!(it.next(), None);
}
