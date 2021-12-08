use crate::Rangetools as _;

#[test]
fn range() {
    let r = ..;
    let u = r.union(3..5);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(5));
    assert!(u.contains(10));
    assert!(u.contains(100));
    assert!(u.contains(1000));
    assert!(!u.is_empty());
}

#[test]
fn range_from() {
    let r = ..;
    let u = r.union(3..);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(5));
    assert!(u.contains(10));
    assert!(u.contains(100));
    assert!(u.contains(1000));
    assert!(!u.is_empty());
}

#[test]
fn range_full() {
    let r = ..;
    let u = r.union(..);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(5));
    assert!(u.contains(10));
    assert!(u.contains(100));
    assert!(u.contains(1000));
    assert!(!u.is_empty());
}

#[test]
fn range_inclusive() {
    let r = ..;
    let u = r.union(3..=5);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(5));
    assert!(u.contains(10));
    assert!(u.contains(100));
    assert!(u.contains(1000));
    assert!(!u.is_empty());
}

#[test]
fn range_to() {
    let r = ..;
    let u = r.union(..5);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(5));
    assert!(u.contains(10));
    assert!(u.contains(100));
    assert!(u.contains(1000));
    assert!(!u.is_empty());
}

#[test]
fn range_to_inclusive() {
    let r = ..;
    let u = r.union(..=5);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(5));
    assert!(u.contains(10));
    assert!(u.contains(100));
    assert!(u.contains(1000));
    assert!(!u.is_empty());
}

#[test]
fn bounded_range() {
    let r = ..;
    let r2 = (1..10).intersection(5..7);
    let u = r.union(r2);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(5));
    assert!(u.contains(10));
    assert!(u.contains(100));
    assert!(u.contains(1000));
    assert!(!u.is_empty());
}

#[test]
fn lower_bounded_range() {
    let r = ..;
    let r2 = (4..).intersection(5..);
    let u = r.union(r2);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(5));
    assert!(u.contains(10));
    assert!(u.contains(100));
    assert!(u.contains(1000));
    assert!(!u.is_empty());
}

#[test]
fn upper_bounded_range() {
    let r = ..;
    let r2 = (..4).intersection(..5);
    let u = r.union(r2);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(5));
    assert!(u.contains(10));
    assert!(u.contains(100));
    assert!(u.contains(1000));
    assert!(!u.is_empty());
}

#[test]
fn unbounded_range() {
    let r = ..;
    let r2 = (..).intersection(..);
    let u = r.union(r2);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(5));
    assert!(u.contains(10));
    assert!(u.contains(100));
    assert!(u.contains(1000));
    assert!(!u.is_empty());
}

#[test]
fn bounded_set() {
    let r = ..;
    let s = (1..5).union(10..12);
    let u = r.union(s);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(5));
    assert!(u.contains(10));
    assert!(u.contains(100));
    assert!(u.contains(1000));
    assert!(!u.is_empty());
}

#[test]
fn lower_bounded_set() {
    let r = ..;
    let s = (..5).union(10..12);
    let u = r.union(s);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(5));
    assert!(u.contains(10));
    assert!(u.contains(100));
    assert!(u.contains(1000));
    assert!(!u.is_empty());
}

#[test]
fn upper_bounded_set() {
    let r = ..;
    let s = (1..5).union(10..);
    let u = r.union(s);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(5));
    assert!(u.contains(10));
    assert!(u.contains(100));
    assert!(u.contains(1000));
    assert!(!u.is_empty());
}

#[test]
fn unbounded_set() {
    let r = ..;
    let s = (..5).union(10..);
    let u = r.union(s);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(5));
    assert!(u.contains(10));
    assert!(u.contains(100));
    assert!(u.contains(1000));
    assert!(!u.is_empty());
}
