use crate::{EmptyRange, Rangetools as _};

#[test]
fn range() {
    let s = (..2).union(2..3);
    let u = s.union(2..4);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(!u.contains(4));
    assert!(!u.contains(5));
    assert!(!u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn range_from() {
    let s = (..2).union(2..3);
    let u = s.union(4..);
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
fn range_full() {
    let s = (..2).union(2..3);
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
    let s = (..2).union(2..3);
    let u = s.union(2..=4);
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
fn range_to() {
    let s = (..2).union(2..3);
    let u = s.union(..4);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(!u.contains(4));
    assert!(!u.contains(5));
    assert!(!u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn range_to_inclusive() {
    let s = (..2).union(2..3);
    let u = s.union(..=4);
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
    let s = (..2).union(2..3);
    let r = (2..4).to_inner();
    let u = s.union(r);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(!u.contains(4));
    assert!(!u.contains(5));
    assert!(!u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn lower_bounded_range() {
    let s = (..2).union(2..3);
    let r = (4..).to_inner();
    let u = s.union(r);
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
fn upper_bounded_range() {
    let s = (..2).union(2..3);
    let r = (..4).to_inner();
    let u = s.union(r);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(!u.contains(4));
    assert!(!u.contains(5));
    assert!(!u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn unbounded_range() {
    let s = (..2).union(2..3);
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
    let s = (..3).union(4..5);
    let u = s.clone().union(EmptyRange::new());
    assert_eq!(s, u);
}

#[test]
fn bounded_set() {
    let s = (..2).union(2..3);
    let s2 = (2..3).union(3..4);
    let u = s.union(s2);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(!u.contains(4));
    assert!(!u.contains(5));
    assert!(!u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn lower_bounded_set() {
    let s = (..2).union(2..3);
    let s2 = (4..5).union(5..);
    let u = s.union(s2);
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
    let s = (..2).union(2..3);
    let s2 = (..3).union(3..4);
    let u = s.union(s2);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(!u.contains(4));
    assert!(!u.contains(5));
    assert!(!u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn unbounded_set() {
    let s = (..2).union(2..3);
    let s2 = (..2).union(5..);
    let u = s.union(s2);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(!u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}
