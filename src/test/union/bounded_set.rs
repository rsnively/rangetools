use crate::{EmptyRange, Rangetools as _};

#[test]
fn range() {
    let s = (1..3).union(4..5);
    let u = s.union(4..6);
    assert_eq!(u.clone().collect::<Vec<_>>(), vec![1, 2, 4, 5]);
    assert!(!u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(!u.contains(6));
    assert!(!u.is_empty());
    assert!(((1..1).union(2..2)).union(0..0).is_empty());
}

#[test]
fn range_from() {
    let s = (1..3).union(4..5);
    let u = s.union(3..);
    assert_eq!(u.clone().take(5).collect::<Vec<_>>(), vec![1, 2, 3, 4, 5]);
    assert!(!u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn range_full() {
    let s = (1..3).union(4..5);
    let u = s.union(..);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn range_inclusive() {
    let s = (1..3).union(4..5);
    let u = s.union(4..=6);
    assert_eq!(u.clone().collect::<Vec<_>>(), vec![1, 2, 4, 5, 6]);
    assert!(!u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
    assert!(((1..1).union(2..2)).union(1..=0).is_empty());
}

#[test]
fn range_to() {
    let s = (1..3).union(4..5);
    let u = s.union(..2);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(u.contains(4));
    assert!(!u.contains(5));
    assert!(!u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn range_to_inclusive() {
    let s = (1..3).union(4..5);
    let u = s.union(..=3);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(!u.contains(5));
    assert!(!u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn bounded_range() {
    let s = (1..3).union(4..5);
    let r = (4..6).intersection(..);
    let u = s.union(r);
    assert_eq!(u.clone().collect::<Vec<_>>(), vec![1, 2, 4, 5]);
    assert!(!u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(!u.contains(6));
    assert!(!u.is_empty());
    let empty_range = (0..0).intersection(..);
    assert!(((1..1).union(2..2)).union(empty_range).is_empty());
}

#[test]
fn lower_bounded_range() {
    let s = (1..3).union(4..5);
    let r = (3..).intersection(..);
    let u = s.union(r);
    assert_eq!(u.clone().take(5).collect::<Vec<_>>(), vec![1, 2, 3, 4, 5]);
    assert!(!u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn upper_bounded_range() {
    let s = (1..3).union(4..5);
    let r = (..2).intersection(..);
    let u = s.union(r);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(u.contains(4));
    assert!(!u.contains(5));
    assert!(!u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn unbounded_range() {
    let s = (1..3).union(4..5);
    let r = (..).to_inner();
    let u = s.union(r);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn empty_range() {
    let s = (1..3).union(4..5);
    let u = s.clone().union(EmptyRange::new());
    assert_eq!(s, u);
}

#[test]
fn bounded_set() {
    let s1 = (1..3).union(4..5);
    let s2 = (2..4).union(5..6);
    let u = s1.union(s2);
    assert_eq!(u.clone().collect::<Vec<_>>(), vec![1, 2, 3, 4, 5]);
    assert!(!u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(!u.contains(6));
    assert!(!u.is_empty());
    let empty_set = (0..0).union(1..1);
    assert!(empty_set.clone().union(empty_set).is_empty());
}

#[test]
fn lower_bounded_set() {
    let s = (1..3).union(4..5);
    let s2 = (0..2).union(5..);
    let u = s.union(s2);
    assert_eq!(u.clone().take(5).collect::<Vec<_>>(), vec![0, 1, 2, 4, 5]);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn upper_bounded_set() {
    let s = (1..3).union(4..5);
    let s2 = (..2).union(5..6);
    let u = s.union(s2);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(!u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn unbounded_set() {
    let s = (1..3).union(4..5);
    let s2 = (..2).union(6..);
    let u = s.union(s2);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(u.contains(4));
    assert!(!u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}
