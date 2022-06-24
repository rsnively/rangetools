use crate::{EmptyRange, Rangetools as _};

#[test]
fn range() {
    let s = (1..3).union(4..6);
    let i = s.clone().intersection(2..6);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![2, 4, 5]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!(s.intersection(10..20).is_empty());
}

#[test]
fn range_from() {
    let s = (1..3).union(4..6);
    let i = s.clone().intersection(2..);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![2, 4, 5]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!(s.intersection(10..).is_empty());
}

#[test]
fn range_full() {
    let s = (1..3).union(4..=5);
    let i = s.intersection(..);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![1, 2, 4, 5]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!((1..1).union(2..2).intersection(..).is_empty());
}

#[test]
fn range_inclusive() {
    let s = (1..3).union(4..6);
    let i = s.clone().intersection(2..=5);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![2, 4, 5]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!(s.intersection(10..=20).is_empty());
}

#[test]
fn range_to() {
    let s = (1..3).union(4..6);
    let i = s.clone().intersection(..5);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![1, 2, 4]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!(s.intersection(..1).is_empty());
}

#[test]
fn range_to_inclusive() {
    let s = (1..3).union(4..6);
    let i = s.clone().intersection(..=5);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![1, 2, 4, 5]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!(s.intersection(..=0).is_empty());
}

#[test]
fn bounded_range() {
    let s = (1..3).union(4..6);
    let r = (2..6).intersection(..);
    let i = s.clone().intersection(r);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![2, 4, 5]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    let r = (10..20).intersection(..);
    assert!(s.intersection(r).is_empty());
}

#[test]
fn lower_bounded_range() {
    let s = (1..3).union(4..6);
    let r = (2..).intersection(1..);
    let i = s.clone().intersection(r);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![2, 4, 5]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    let r = (10..).intersection(20..);
    assert!(s.intersection(r).is_empty());
}

#[test]
fn upper_bounded_range() {
    let s = (1..3).union(4..6);
    let r = (..5).intersection(..6);
    let i = s.clone().intersection(r);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![1, 2, 4]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    let r = (..1).intersection(..2);
    assert!(s.intersection(r).is_empty());
}

#[test]
fn unbounded_range() {
    let s = (1..3).union(4..=5);
    let r = (..).to_inner();
    let i = s.intersection(r);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![1, 2, 4, 5]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!((1..1).union(2..2).intersection(r).is_empty());
}

#[test]
fn empty_range() {
    let s = (1..3).union(4..5);
    let r = EmptyRange::new();
    let i = s.intersection(r);
    assert!(i.is_empty());
    assert_eq!(i.collect::<Vec<_>>(), vec![]);
}

#[test]
fn bounded_set() {
    let s1 = (1..3).union(4..6);
    let s2 = (2..=2).union(4..5);
    let i = s1.clone().intersection(s2);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![2, 4]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    let s2 = (10..20).union(30..40);
    assert!(s1.intersection(s2).is_empty());
}

#[test]
fn lower_bounded_set() {
    let s1 = (1..3).union(4..6);
    let s2 = (2..).union(3..);
    let i = s1.clone().intersection(s2);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![2, 4, 5]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    let s2 = (10..).union(20..);
    assert!(s1.intersection(s2).is_empty());
}

#[test]
fn upper_bounded_set() {
    let s1 = (1..2).union(3..5);
    let s2 = (..3).union(4..6);
    let i = s1.clone().intersection(s2);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![1, 4]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(!i.contains(3));
    assert!(i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    let s2 = (..1).union(..0);
    assert!(s1.intersection(s2).is_empty());
}

#[test]
fn unbounded_set() {
    let s = (1..3).union(4..=5);
    let r = (..=1).union(2..3).union(5..);
    let i = s.clone().intersection(r);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![1, 2, 5]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    let r = (..1).union(3..4).union(6..);
    assert!(s.intersection(r).is_empty());
}
