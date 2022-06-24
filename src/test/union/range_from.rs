use crate::{EmptyRange, Rangetools as _};

#[test]
fn range() {
    let u = (3..).union(2..4);
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
    let u = (3..).union(4..);
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
    let u = (3..).union(..);
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
    let u = (3..).union(1..=2);
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
    let u = (3..).union(..2);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(!u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());

    let full = (3..).union(..4);
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
    let u = (3..).union(..=1);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(!u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());

    let full = (3..).union(..=4);
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
    let r = (2..4).intersection(..);
    let u = (3..).union(r);
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
    let r = (4..).intersection(..);
    let u = (3..).union(r);
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
    let r = (..2).intersection(..);
    let u = (3..).union(r);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(!u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());

    let r = (..4).intersection(..);
    let full = (3..).union(r);
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
    let r = (..).intersection(..);
    let u = (3..).union(r);
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
    let r = 3..;
    let u = r.clone().union(EmptyRange::new());
    assert_eq!(r, u);
}

#[test]
fn bounded_set() {
    let s = (2..4).union(4..5);
    let u = (3..).union(s);
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
    let s = (2..3).union(4..);
    let u = (3..).union(s);
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
    let s = (..1).union(2..3);
    let u = (4..).union(s);
    assert!(u.contains(0));
    assert!(!u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());

    let s = (..4).union(5..6);
    let full = (3..).union(s);
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
    let s = (..1).union(4..);
    let u = (3..).union(s);
    assert!(u.contains(0));
    assert!(!u.contains(1));
    assert!(!u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}
