use crate::{EmptyRange, Rangetools as _};

#[test]
fn range() {
    let u = (0..=2).union(5..7);
    assert_eq!(u.clone().collect::<Vec<_>>(), vec![0, 1, 2, 5, 6]);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(!u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.contains(7));
    assert!(!u.contains(8));
    assert!(!u.is_empty());
    assert!((1..=0).union(1..1).is_empty());
}

#[test]
fn range_from() {
    let u = (1..=2).union(2..);
    assert_eq!(u.clone().take(3).collect::<Vec<_>>(), vec![1, 2, 3]);
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
    let u = (1..=2).union(..);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(u.contains(7));
    assert!(u.contains(8));
    assert!(!u.is_empty());
}

#[test]
fn range_inclusive() {
    let u = (1..=2).union(5..=7);
    assert_eq!(u.clone().collect::<Vec<_>>(), vec![1, 2, 5, 6, 7]);
    assert!(!u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(!u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(u.contains(7));
    assert!(!u.contains(8));
    assert!(!u.is_empty());
    assert!((1..=0).union(1..=0).is_empty());
}

#[test]
fn range_to() {
    let u = (1..=2).union(..2);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(!u.contains(4));
    assert!(!u.contains(5));
    assert!(!u.contains(6));
    assert!(!u.contains(7));
    assert!(!u.contains(8));
    assert!(!u.is_empty());
}

#[test]
fn range_to_inclusive() {
    let u = (1..=2).union(..=4);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(!u.contains(5));
    assert!(!u.contains(6));
    assert!(!u.contains(7));
    assert!(!u.contains(8));
    assert!(!u.is_empty());
}

#[test]
fn bounded_range() {
    let r = (5..7).intersection(..);
    let u = (1..=2).union(r);
    assert_eq!(u.clone().collect::<Vec<_>>(), vec![1, 2, 5, 6]);
    assert!(!u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(!u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.contains(7));
    assert!(!u.contains(8));
    assert!(!u.is_empty());
    let r = (1..1).intersection(..);
    assert!((1..=0).union(r).is_empty());
}

#[test]
fn lower_bounded_range() {
    let r = (..).intersection(2..);
    let u = (1..=2).union(r);
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
    let r = (..1).intersection(..);
    let u = (1..=2).union(r);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(!u.contains(4));
    assert!(!u.contains(5));
    assert!(!u.contains(6));
    assert!(!u.contains(7));
    assert!(!u.contains(8));
    assert!(!u.is_empty());
}

#[test]
fn unbounded_range() {
    let ur = (..).intersection(..);
    let u = (1..=2).union(ur);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
    assert!(!(1..1).union(ur).is_empty());
}

#[test]
fn empty_range() {
    let r = 1..=2;
    let u = r.clone().union(EmptyRange::new());
    assert_eq!(r, u);
}

#[test]
fn bounded_set() {
    let s = (5..7).union(2..4);
    let u = (1..=2).union(s);
    assert_eq!(u.clone().collect::<Vec<_>>(), vec![1, 2, 3, 5, 6]);
    assert!(!u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(!u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.contains(7));
    assert!(!u.contains(8));
    assert!(!u.is_empty());
    let s = (1..1).union(2..2);
    assert!((0..0).union(s).is_empty());
}

#[test]
fn lower_bounded_set() {
    let s = (2..3).union(5..);
    let u = (1..=2).union(s);
    assert_eq!(u.clone().take(5).collect::<Vec<_>>(), vec![1, 2, 5, 6, 7]);
    assert!(!u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(!u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn upper_bounded_set() {
    let s = (..1).union(4..5);
    let u = (1..=2).union(s);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(u.contains(4));
    assert!(!u.contains(5));
    assert!(!u.contains(6));
    assert!(!u.contains(7));
    assert!(!u.contains(8));
    assert!(!u.is_empty());
}

#[test]
fn unbounded_set() {
    let s = (..2).union(5..);
    let u = (1..=2).union(s);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(!u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}
