use crate::{EmptyRange, Rangetools as _};

#[test]
fn range() {
    let s = (3..4).union(4..);
    let u = s.union(2..4);
    assert_eq!(u.clone().take(5).collect::<Vec<_>>(), vec![2, 3, 4, 5, 6]);
    assert!(!u.contains(0));
    assert!(!u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn range_from() {
    let s = (3..4).union(4..);
    let u = s.union(4..);
    assert_eq!(u.clone().take(5).collect::<Vec<_>>(), vec![3, 4, 5, 6, 7]);
    assert!(!u.contains(0));
    assert!(!u.contains(1));
    assert!(!u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn range_full() {
    let s = (3..4).union(4..);
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
    let s = (3..4).union(4..);
    let u = s.union(1..=2);
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
fn range_to() {
    let s = (3..4).union(4..);
    let u = s.clone().union(..2);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(!u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());

    let full = s.union(..4);
    assert!(!full.is_empty());
    assert!(full.contains(0));
    assert!(full.contains(1));
    assert!(full.contains(2));
    assert!(full.contains(3));
    assert!(full.contains(4));
    assert!(full.contains(5));
}

#[test]
fn range_to_inclusive() {
    let s = (3..4).union(4..);
    let u = s.clone().union(..=1);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(!u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());

    let full = s.union(..=4);
    assert!(!full.is_empty());
    assert!(full.contains(0));
    assert!(full.contains(1));
    assert!(full.contains(2));
    assert!(full.contains(3));
    assert!(full.contains(4));
    assert!(full.contains(5));
}

#[test]
fn bounded_range() {
    let s = (3..4).union(4..);
    let r = (2..4).intersection(..);
    let u = s.union(r);
    assert_eq!(u.clone().take(5).collect::<Vec<_>>(), vec![2, 3, 4, 5, 6]);
    assert!(!u.contains(0));
    assert!(!u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn lower_bounded_range() {
    let s = (3..4).union(4..);
    let r = (4..).intersection(..);
    let u = s.union(r);
    assert_eq!(u.clone().take(5).collect::<Vec<_>>(), vec![3, 4, 5, 6, 7]);
    assert!(!u.contains(0));
    assert!(!u.contains(1));
    assert!(!u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn upper_bounded_range() {
    let s = (3..4).union(4..);
    let r = (..2).intersection(..);
    let u = s.clone().union(r);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(!u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());

    let r = (..4).intersection(..);
    let full = s.union(r);
    assert!(!full.is_empty());
    assert!(full.contains(0));
    assert!(full.contains(1));
    assert!(full.contains(2));
    assert!(full.contains(3));
    assert!(full.contains(4));
    assert!(full.contains(5));
}

#[test]
fn unbounded_range() {
    let s = (3..4).union(4..);
    let r = (..).intersection(..);
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
    let s = (1..3).union(4..);
    let u = s.clone().union(EmptyRange::new());
    assert_eq!(s, u);
}

#[test]
fn bounded_set() {
    let s = (3..4).union(4..);
    let s2 = (2..4).union(4..5);
    let u = s.union(s2);
    assert_eq!(u.clone().take(5).collect::<Vec<_>>(), vec![2, 3, 4, 5, 6]);
    assert!(!u.contains(0));
    assert!(!u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn lower_bounded_set() {
    let s = (3..4).union(4..);
    let s2 = (2..3).union(4..);
    let u = s.union(s2);
    assert_eq!(u.clone().take(5).collect::<Vec<_>>(), vec![2, 3, 4, 5, 6]);
    assert!(!u.contains(0));
    assert!(!u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn upper_bounded_set() {
    let s = (4..5).union(5..);
    let s2 = (..1).union(2..3);
    let u = s.clone().union(s2);
    assert!(u.contains(0));
    assert!(!u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());

    let s2 = (..4).union(5..6);
    let full = s.union(s2);
    assert!(!full.is_empty());
    assert!(full.contains(0));
    assert!(full.contains(1));
    assert!(full.contains(2));
    assert!(full.contains(3));
    assert!(full.contains(4));
    assert!(full.contains(5));
}

#[test]
fn unbounded_set() {
    let s = (3..4).union(4..);
    let s2 = (..1).union(4..);
    let u = s.union(s2);
    assert!(u.contains(0));
    assert!(!u.contains(1));
    assert!(!u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}
