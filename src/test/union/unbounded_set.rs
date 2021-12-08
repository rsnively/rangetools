use crate::Rangetools as _;

#[test]
fn range() {
    let s = (..2).union(3..4).union(5..);
    let u = s.union(2..4);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(!u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn range_from() {
    let s = (..2).union(3..4).union(5..);
    let u = s.union(4..);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(!u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn range_full() {
    let s = (..2).union(3..4).union(5..);
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
    let s = (..2).union(3..4).union(5..);
    let u = s.union(2..=3);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(!u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn range_to() {
    let s = (..2).union(3..4).union(5..);
    let u = s.union(..4);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(!u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn range_to_inclusive() {
    let s = (..2).union(3..4).union(5..);
    let u = s.union(..=3);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(!u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn bounded_range() {
    let s = (..2).union(3..4).union(5..);
    let r = (2..5).intersection(2..3);
    let u = s.union(r);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(!u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn lower_bounded_range() {
    let s = (..2).union(3..4).union(5..);
    let r = (4..).intersection(3..);
    let u = s.union(r);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(!u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn upper_bounded_range() {
    let s = (..2).union(3..4).union(5..);
    let r = (..4).intersection(..5);
    let u = s.union(r);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(!u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn unbounded_range() {
    let s = (..2).union(3..4).union(5..);
    let u = s.union((..).to_inner());
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
fn bounded_set() {
    let s = (..2).union(3..4).union(5..);
    let s2 = (2..3).union(3..4);
    let u = s.union(s2);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(!u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn lower_bounded_set() {
    let s = (..2).union(3..4).union(5..);
    let s2 = (4..).union(1..2);
    let u = s.union(s2);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(!u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn upper_bounded_set() {
    let s = (..2).union(3..4).union(5..);
    let s2 = (..4).union(5..10);
    let u = s.union(s2);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(!u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn unbounded_set() {
    let s = (..2).union(3..4).union(5..);
    let s2 = (..1).union(2..3).union(6..);
    let u = s.union(s2);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(!u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}
