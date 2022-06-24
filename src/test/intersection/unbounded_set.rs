use crate::{EmptyRange, Rangetools as _};

#[test]
fn range() {
    let s = (..2).union(3..4).union(5..);
    let i = s.clone().intersection(1..6);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![1, 3, 5]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.contains(7));
    assert!(!i.is_empty());
    assert!(s.intersection(2..3).is_empty());
}

#[test]
fn range_from() {
    let s = (..2).union(3..4).union(5..);
    let i = s.clone().intersection(3..);
    assert_eq!(i.clone().take(3).collect::<Vec<_>>(), vec![3, 5, 6]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(i.contains(6));
    assert!(i.contains(7));
    assert!(!i.is_empty());
}

#[test]
fn range_full() {
    let s = (..2).union(3..4).union(5..);
    let i = s.intersection(..);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(i.contains(6));
    assert!(i.contains(7));
    assert!(!i.is_empty());
}

#[test]
fn range_inclusive() {
    let s = (..2).union(3..4).union(5..);
    let i = s.clone().intersection(1..=6);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![1, 3, 5, 6]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(i.contains(6));
    assert!(!i.contains(7));
    assert!(!i.is_empty());
    assert!(s.intersection(2..=2).is_empty());
}

#[test]
fn range_to() {
    let s = (..2).union(3..4).union(5..);
    let i = s.intersection(..6);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.contains(7));
    assert!(!i.is_empty());
}

#[test]
fn range_to_inclusive() {
    let s = (..2).union(3..4).union(5..);
    let i = s.intersection(..=5);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.contains(7));
    assert!(!i.is_empty());
}

#[test]
fn bounded_range() {
    let s = (..2).union(3..4).union(5..);
    let r = (1..=3).intersection(2..5);
    let i = s.clone().intersection(r);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![3]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.contains(7));
    assert!(!i.is_empty());
    let r = (2..3).intersection(..);
    assert!(s.intersection(r).is_empty());
}

#[test]
fn lower_bounded_range() {
    let s = (..2).union(3..4).union(5..);
    let r = (3..).to_inner();
    let i = s.clone().intersection(r);
    assert_eq!(i.clone().take(3).collect::<Vec<_>>(), vec![3, 5, 6]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(i.contains(6));
    assert!(i.contains(7));
    assert!(!i.is_empty());
}

#[test]
fn upper_bounded_range() {
    let s = (..2).union(3..4).union(5..);
    let r = (..6).to_inner();
    let i = s.intersection(r);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.contains(7));
    assert!(!i.is_empty());
}

#[test]
fn unbounded_range() {
    let s = (..2).union(3..4).union(5..);
    let r = (..).to_inner();
    let i = s.intersection(r);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(i.contains(6));
    assert!(i.contains(7));
    assert!(!i.is_empty());
}

#[test]
fn empty_range() {
    let s = (..2).union(3..4).union(5..);
    let r = EmptyRange::new();
    let i = s.intersection(r);
    assert!(i.is_empty());
    assert_eq!(i.collect::<Vec<_>>(), vec![]);
}

#[test]
fn bounded_set() {
    let s1 = (..2).union(3..4).union(5..);
    let s2 = (1..3).union(4..=5);
    let i = s1.clone().intersection(s2);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![1, 5]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.contains(7));
    assert!(!i.is_empty());
    let s2 = (2..3).union(4..5);
    assert!(s1.intersection(s2).is_empty());
}

#[test]
fn lower_bounded_set() {
    let s = (..2).union(3..4).union(5..);
    let s2 = (3..).union(5..6);
    let i = s.clone().intersection(s2);
    assert_eq!(i.clone().take(3).collect::<Vec<_>>(), vec![3, 5, 6]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(i.contains(6));
    assert!(i.contains(7));
    assert!(!i.is_empty());
}

#[test]
fn upper_bounded_set() {
    let s = (..2).union(3..4).union(5..);
    let s2 = (..6).union(7..8);
    let i = s.intersection(s2);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(!i.contains(6));
    assert!(i.contains(7));
    assert!(!i.is_empty());
}

#[test]
fn unbounded_set() {
    let s = (..2).union(3..4).union(5..);
    let s2 = (..4).union(7..);
    let i = s.intersection(s2);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(i.contains(7));
    assert!(!i.is_empty());
}
