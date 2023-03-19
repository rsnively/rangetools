use crate::{BoundedSet, LowerBoundedSet, Rangetools};

#[test]
fn next() {
    let s1: LowerBoundedSet<i32> = (4..).union(BoundedSet::empty());
    let mut i1 = s1.into_iter();
    assert_eq!(i1.next(), Some(4));
    assert_eq!(i1.next(), Some(5));

    let s2: LowerBoundedSet<i32> = (1..2).union(4..);
    let mut i2 = s2.into_iter();
    assert_eq!(i2.next(), Some(1));
    assert_eq!(i2.next(), Some(4));
}

#[test]
fn size_hint() {
    let hint = (usize::MAX, None);
    let s1: LowerBoundedSet<i32> = (4..).union(BoundedSet::empty());
    assert_eq!(s1.into_iter().size_hint(), hint);

    let s2: LowerBoundedSet<i32> = (1..2).union(4..);
    assert_eq!(s2.into_iter().size_hint(), hint);
}

#[test]
fn nth() {
    let s1: LowerBoundedSet<i32> = (4..).union(BoundedSet::empty());
    let mut i1 = s1.into_iter();
    assert_eq!(i1.nth(1), Some(5));
    assert_eq!(i1.nth(2), Some(8));

    let s2: LowerBoundedSet<i32> = (1..2).union(4..);
    let mut i2 = s2.into_iter();
    assert_eq!(i2.nth(1), Some(4));
    assert_eq!(i2.nth(2), Some(7));
}
