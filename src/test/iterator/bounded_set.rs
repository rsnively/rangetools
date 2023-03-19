use crate::{BoundedSet, Rangetools};

#[test]
fn next() {
    let s1: BoundedSet<i32> = (0..1).union(3..4);
    let mut i1 = s1.into_iter();
    assert_eq!(i1.next(), Some(0));
    assert_eq!(i1.next(), Some(3));
    assert_eq!(i1.next(), None);

    let s2: BoundedSet<i32> = BoundedSet::empty();
    let mut i2 = s2.into_iter();
    assert_eq!(i2.next(), None);
}

#[test]
fn size_hint() {
    let s1: BoundedSet<i32> = (0..1).union(3..4);
    assert_eq!(s1.into_iter().size_hint(), (2, Some(2)));

    let s2: BoundedSet<i32> = BoundedSet::empty();
    assert_eq!(s2.into_iter().size_hint(), (0, Some(0)));
}

#[test]
fn count() {
    let s1: BoundedSet<i32> = (0..1).union(3..4);
    assert_eq!(s1.into_iter().count(), 2);

    let s2: BoundedSet<i32> = BoundedSet::empty();
    assert_eq!(s2.into_iter().count(), 0);
}

#[test]
fn last() {
    let s1: BoundedSet<i32> = (0..1).union(3..4);
    assert_eq!(s1.into_iter().last(), Some(3));

    let s2: BoundedSet<i32> = BoundedSet::empty();
    assert_eq!(s2.into_iter().last(), None);
}

#[test]
fn nth() {
    let s1: BoundedSet<i32> = (0..1).union(3..4);
    assert_eq!(s1.into_iter().nth(1), Some(3));

    let s2: BoundedSet<i32> = BoundedSet::empty();
    assert_eq!(s2.into_iter().nth(1), None);
}

#[test]
fn min() {
    let s1: BoundedSet<i32> = (0..1).union(3..4);
    assert_eq!(s1.into_iter().min(), Some(0));

    let s2: BoundedSet<i32> = BoundedSet::empty();
    assert_eq!(s2.into_iter().min(), None);
}

#[test]
fn max() {
    let s1: BoundedSet<i32> = (0..1).union(3..4);
    assert_eq!(s1.into_iter().max(), Some(3));

    let s2: BoundedSet<i32> = BoundedSet::empty();
    assert_eq!(s2.into_iter().max(), None);
}

#[test]
fn next_back() {
    let s1: BoundedSet<i32> = (0..1).union(3..4);
    let mut i1 = s1.into_iter();
    assert_eq!(i1.next_back(), Some(3));
    assert_eq!(i1.next_back(), Some(0));
    assert_eq!(i1.next_back(), None);

    let s2: BoundedSet<i32> = BoundedSet::empty();
    let mut i2 = s2.into_iter();
    assert_eq!(i2.next_back(), None);
}

#[test]
fn nth_back() {
    let s1: BoundedSet<i32> = (0..1).union(3..4);
    assert_eq!(s1.into_iter().nth_back(1), Some(0));

    let s2: BoundedSet<i32> = BoundedSet::empty();
    assert_eq!(s2.into_iter().nth_back(1), None);
}
