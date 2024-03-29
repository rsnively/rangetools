use crate::{EmptyRange, Rangetools as _};

#[test]
fn range() {
    let r = (..3).intersection(..);
    let i = r.intersection(2..4);
    assert_eq!(i.into_iter().collect::<Vec<_>>(), vec![2]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    assert!(r.intersection(5..10).is_empty());
}

#[test]
fn range_from() {
    let r = (..3).intersection(..);
    let i = r.intersection(2..);
    assert_eq!(i.into_iter().collect::<Vec<_>>(), vec![2]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    assert!(r.intersection(5..).is_empty());
}

#[test]
fn range_full() {
    let r = (..3).intersection(..);
    let i = r.intersection(..);
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
    let r = (..3).intersection(..);
    let i = r.intersection(2..=4);
    assert_eq!(i.into_iter().collect::<Vec<_>>(), vec![2]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    assert!(r.intersection(5..=10).is_empty());
}

#[test]
fn range_to() {
    let r = (..3).intersection(..);
    let i = r.intersection(..2);
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
    let r = (..3).intersection(..);
    let i = r.intersection(..=2);
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
    let r1 = (..3).intersection(..);
    let r2 = (2..4).intersection(..);
    let i = r1.intersection(r2);
    assert_eq!(i.into_iter().collect::<Vec<_>>(), vec![2]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    let r3 = (5..10).intersection(..);
    assert!(r1.intersection(r3).is_empty());
}

#[test]
fn lower_bounded_range() {
    let r1 = (..3).intersection(..);
    let r2 = (2..).intersection(..);
    let i = r1.intersection(r2);
    assert_eq!(i.into_iter().collect::<Vec<_>>(), vec![2]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    let r3 = (5..).intersection(..);
    assert!(r1.intersection(r3).is_empty());
}

#[test]
fn upper_bounded_range() {
    let r1 = (..3).intersection(..);
    let r2 = (..2).intersection(..);
    let i = r1.intersection(r2);
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
    let r1 = (..3).intersection(..);
    let r2 = (..).intersection(..);
    let i = r1.intersection(r2);
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
    let r1 = (..3).intersection(..);
    let r2 = EmptyRange::new();
    let i = r1.intersection(r2);
    assert!(i.is_empty());
    assert_eq!(i.into_iter().collect::<Vec<_>>(), vec![]);
}

#[test]
fn bounded_set() {
    let r = (..3).to_inner();
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
    let r = (..3).to_inner();
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
    let r = (..3).to_inner();
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
    let r = (..3).to_inner();
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
