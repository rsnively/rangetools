use crate::{EmptyRange, Rangetools as _};

#[test]
fn range() {
    let r = EmptyRange::<i32>::new();
    let i = r.intersection(1..3);
    assert!(i.is_empty());
    assert_eq!(i.collect::<Vec<_>>(), vec![]);
}

#[test]
fn range_from() {
    let r = EmptyRange::<i32>::new();
    let i = r.intersection(1..);
    assert!(i.is_empty());
    assert_eq!(i.collect::<Vec<_>>(), vec![]);
}

#[test]
fn range_full() {
    let r = EmptyRange::<i32>::new();
    let i = r.intersection(..);
    assert!(i.is_empty());
    assert_eq!(i.collect::<Vec<_>>(), vec![]);
}

#[test]
fn range_inclusive() {
    let r = EmptyRange::<i32>::new();
    let i = r.intersection(1..=3);
    assert!(i.is_empty());
    assert_eq!(i.collect::<Vec<_>>(), vec![]);
}

#[test]
fn range_to() {
    let r = EmptyRange::<i32>::new();
    let i = r.intersection(..3);
    assert!(i.is_empty());
    assert_eq!(i.collect::<Vec<_>>(), vec![]);
}

#[test]
fn range_to_inclusive() {
    let r = EmptyRange::<i32>::new();
    let i = r.intersection(..=3);
    assert!(i.is_empty());
    assert_eq!(i.collect::<Vec<_>>(), vec![]);
}

#[test]
fn bounded_range() {
    let r1 = EmptyRange::<i32>::new();
    let r2 = (1..3).intersection(..);
    let i = r1.intersection(r2);
    assert!(i.is_empty());
    assert_eq!(i.collect::<Vec<_>>(), vec![]);
}

#[test]
fn lower_bounded_range() {
    let r1 = EmptyRange::<i32>::new();
    let r2 = (3..).intersection(..);
    let i = r1.intersection(r2);
    assert!(i.is_empty());
    assert_eq!(i.collect::<Vec<_>>(), vec![]);
}
#[test]
fn upper_bounded_range() {
    let r1 = EmptyRange::<i32>::new();
    let r2 = (..3).intersection(..);
    let i = r1.intersection(r2);
    assert!(i.is_empty());
    assert_eq!(i.collect::<Vec<_>>(), vec![]);
}
#[test]
fn unbounded_range() {
    let r1 = EmptyRange::<i32>::new();
    let r2 = (..).intersection(..);
    let i = r1.intersection(r2);
    assert!(i.is_empty());
    assert_eq!(i.collect::<Vec<_>>(), vec![]);
}

#[test]
fn empty_range() {
    let r1 = EmptyRange::<i32>::new();
    let r2 = EmptyRange::<i32>::new();
    let i = r1.intersection(r2);
    assert!(i.is_empty());
    assert_eq!(i.collect::<Vec<_>>(), vec![]);
}

#[test]
fn bounded_set() {
    let r = EmptyRange::<i32>::new();
    let s = (1..3).union(4..6);
    let i = r.intersection(s);
    assert!(i.is_empty());
    assert_eq!(i.collect::<Vec<_>>(), vec![]);
}

#[test]
fn lower_bounded_set() {
    let r = EmptyRange::<i32>::new();
    let s = (3..).union(0..2);
    let i = r.intersection(s);
    assert!(i.is_empty());
    assert_eq!(i.collect::<Vec<_>>(), vec![]);
}

#[test]
fn upper_bounded_set() {
    let r = EmptyRange::<i32>::new();
    let s = (..3).union(4..5);
    let i = r.intersection(s);
    assert!(i.is_empty());
    assert_eq!(i.collect::<Vec<_>>(), vec![]);
}

#[test]
fn unbounded_set() {
    let r = EmptyRange::<i32>::new();
    let s = (2..3).union(..1).union(4..);
    let i = r.intersection(s);
    assert!(i.is_empty());
    assert_eq!(i.collect::<Vec<_>>(), vec![]);
}
