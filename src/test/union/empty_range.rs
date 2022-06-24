use crate::{EmptyRange, Rangetools as _};

#[test]
fn range() {
    let r = 1..3;
    let u = EmptyRange::<i32>::new().union(r.clone());
    assert_eq!(r, u);
}

#[test]
fn range_from() {
    let r = 1..;
    let u = EmptyRange::<i32>::new().union(r.clone());
    assert_eq!(r, u);
}

#[test]
fn range_full() {
    let r = ..;
    let u = EmptyRange::<i32>::new().union(r);
    assert_eq!(r, u);
}

#[test]
fn range_inclusive() {
    let r = 1..=3;
    let u = EmptyRange::<i32>::new().union(r.clone());
    assert_eq!(r, u);
}

#[test]
fn range_to() {
    let r = ..3;
    let u = EmptyRange::<i32>::new().union(r);
    assert_eq!(r, u);
}

#[test]
fn range_to_inclusive() {
    let r = ..=3;
    let u = EmptyRange::<i32>::new().union(r);
    assert_eq!(r, u);
}

#[test]
fn bounded_range() {
    let r = (1..3).to_inner();
    let u = EmptyRange::<i32>::new().union(r);
    assert_eq!(r, u);
}

#[test]
fn lower_bounded_range() {
    let r = (1..).to_inner();
    let u = EmptyRange::<i32>::new().union(r);
    assert_eq!(r, u);
}

#[test]
fn upper_bounded_range() {
    let r = (1..3).to_inner();
    let u = EmptyRange::<i32>::new().union(r);
    assert_eq!(r, u);
}

#[test]
fn unbounded_range() {
    let r = (..).to_inner();
    let u = EmptyRange::<i32>::new().union(r);
    assert_eq!(r, u);
}

#[test]
fn empty_range() {
    let r = EmptyRange::<i32>::new();
    let u = EmptyRange::<i32>::new().union(r);
    assert_eq!(r, u);
}

#[test]
fn bounded_set() {
    let s = (1..3).union(2..=5);
    let u = EmptyRange::<i32>::new().union(s.clone());
    assert_eq!(s, u);
}

#[test]
fn lower_bounded_set() {
    let s = (1..3).union(2..);
    let u = EmptyRange::<i32>::new().union(s.clone());
    assert_eq!(s, u);
}

#[test]
fn upper_bounded_set() {
    let s = (..3).union(2..=5);
    let u = EmptyRange::<i32>::new().union(s.clone());
    assert_eq!(s, u);
}

#[test]
fn unbounded_set() {
    let s = (..3).union(2..);
    let u = EmptyRange::<i32>::new().union(s.clone());
    assert_eq!(s, u);
}
