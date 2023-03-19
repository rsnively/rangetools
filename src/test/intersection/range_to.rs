use crate::{EmptyRange, Rangetools as _};

#[test]
fn range() {
    let i = (..3).intersection(2..4);
    assert_eq!(i.into_iter().collect::<Vec<_>>(), vec![2]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    assert!((..3).intersection(5..10).is_empty());
}

#[test]
fn range_from() {
    let i = (..3).intersection(2..);
    assert_eq!(i.into_iter().collect::<Vec<_>>(), vec![2]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    assert!((..3).intersection(5..).is_empty());
}

#[test]
fn range_full() {
    let i = (..3).intersection(..);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
}

#[test]
fn range_inclusive() {
    let i = (..3).intersection(2..=4);
    assert_eq!(i.into_iter().collect::<Vec<_>>(), vec![2]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    assert!((..3).intersection(5..=10).is_empty());
}

#[test]
fn range_to() {
    let i = (..3).intersection(..2);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
}

#[test]
fn range_to_inclusive() {
    let i = (..3).intersection(..=2);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
}

#[test]
fn bounded_range() {
    let r = (2..4).intersection(..);
    let i = (..3).intersection(r);
    assert_eq!(i.into_iter().collect::<Vec<_>>(), vec![2]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    let r2 = (5..10).intersection(..);
    assert!((..3).intersection(r2).is_empty());
}

#[test]
fn lower_bounded_range() {
    let r = (2..).intersection(..);
    let i = (..3).intersection(r);
    assert_eq!(i.into_iter().collect::<Vec<_>>(), vec![2]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    let r2 = (5..).intersection(..);
    assert!((..3).intersection(r2).is_empty());
}

#[test]
fn upper_bounded_range() {
    let r = (..2).intersection(..);
    let i = (..3).intersection(r);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
}

#[test]
fn unbounded_range() {
    let r = (..).intersection(..);
    let i = (..3).intersection(r);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
}

#[test]
fn empty_range() {
    let r1 = ..3;
    let r2 = EmptyRange::new();
    let i = r1.intersection(r2);
    assert!(i.is_empty());
    assert_eq!(i.into_iter().collect::<Vec<_>>(), vec![]);
}

#[test]
fn bounded_set() {
    let r = ..3;
    let s = (1..2).union(3..4);
    let i = r.intersection(s);
    assert_eq!(i.clone().into_iter().collect::<Vec<_>>(), vec![1]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    let s2 = (3..4).union(5..6);
    assert!(r.intersection(s2).is_empty());
}

#[test]
fn lower_bounded_set() {
    let r = ..3;
    let s = (0..1).union(2..);
    let i = r.intersection(s);
    assert_eq!(i.clone().into_iter().collect::<Vec<_>>(), vec![0, 2]);
    assert!(i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    let s2 = (5..).to_set();
    assert!(r.intersection(s2).is_empty());
}

#[test]
fn upper_bounded_set() {
    let r = ..3;
    let s = (..2).union(3..4);
    let i = r.intersection(s);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
}

#[test]
fn unbounded_set() {
    let r = ..3;
    let s = (..1).union(2..);
    let i = r.intersection(s);
    assert!(i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
}
