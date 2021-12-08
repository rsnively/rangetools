use crate::Rangetools as _;

#[test]
fn range() {
    let r = ..=2;
    let u = r.union(2..4);
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
    let r = ..=2;
    let u = r.union(4..);
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
    let r = ..=2;
    let u = r.union(..);
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
    let r = ..=2;
    let u = r.union(2..=4);
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
    let r = ..=2;
    let u = r.union(..4);
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
    let r = ..=2;
    let u = r.union(..=4);
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
    let r = ..=2;
    let r2 = (2..4).to_inner();
    let u = r.union(r2);
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
    let r = ..=2;
    let r2 = (4..).to_inner();
    let u = r.union(r2);
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
    let r = ..=2;
    let r2 = (..4).to_inner();
    let u = r.union(r2);
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
    let r = ..=2;
    let r2 = (..).intersection(..);
    let u = r.union(r2);
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
    let r = ..=2;
    let s = (2..3).union(3..4);
    let u = r.union(s);
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
    let r = ..=2;
    let s = (4..5).union(5..);
    let u = r.union(s);
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
    let r = ..=2;
    let s = (..3).union(3..4);
    let u = r.union(s);
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
    let r = ..=2;
    let s = (..2).union(5..);
    let u = r.union(s);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(!u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}
