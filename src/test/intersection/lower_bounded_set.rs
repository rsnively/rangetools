use crate::{EmptyRange, Rangetools as _};

#[test]
fn range() {
    let s = (1..2).union(3..);
    let i = s.clone().intersection(1..4);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![1, 3]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    assert!(s.intersection(2..3).is_empty());
}

#[test]
fn range_from() {
    let s = (1..2).union(3..);
    let i = s.intersection(1..);
    assert_eq!(i.clone().take(3).collect::<Vec<_>>(), vec![1, 3, 4]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.is_empty());
}

#[test]
fn range_full() {
    let s = (1..2).union(3..);
    let i = s.intersection(..);
    assert_eq!(i.clone().take(3).collect::<Vec<_>>(), vec![1, 3, 4]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.is_empty());
}

#[test]
fn range_inclusive() {
    let s = (1..2).union(3..);
    let i = s.clone().intersection(1..=4);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![1, 3, 4]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    assert!(s.intersection(2..=2).is_empty());
}

#[test]
fn range_to() {
    let s = (1..2).union(3..);
    let i = s.clone().intersection(..5);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![1, 3, 4]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    assert!(s.intersection(..1).is_empty());
}

#[test]
fn range_to_inclusive() {
    let s = (1..2).union(3..);
    let i = s.clone().intersection(..=5);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![1, 3, 4, 5]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.is_empty());
    assert!(s.intersection(..=0).is_empty());
}

#[test]
fn bounded_range() {
    let s = (1..2).union(3..);
    let r = (0..8).intersection(1..5);
    let i = s.clone().intersection(r);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![1, 3, 4]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    let r = (0..1).intersection(0..3);
    assert!(s.intersection(r).is_empty());
}

#[test]
fn lower_bounded_range() {
    let s = (1..2).union(3..);
    let r = (1..).intersection(..);
    let i = s.intersection(r);
    assert_eq!(i.clone().take(3).collect::<Vec<_>>(), vec![1, 3, 4]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.is_empty());
}

#[test]
fn upper_bounded_range() {
    let s = (1..2).union(3..);
    let r = (..5).intersection(..);
    let i = s.clone().intersection(r);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![1, 3, 4]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    let r = (0..1).intersection(..);
    assert!(s.intersection(r).is_empty());
}

#[test]
fn unbounded_range() {
    let s = (1..2).union(3..);
    let r = (..).to_inner();
    let i = s.intersection(r);
    assert_eq!(i.clone().take(3).collect::<Vec<_>>(), vec![1, 3, 4]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.is_empty());
}

#[test]
fn empty_range() {
    let s = (1..2).union(3..);
    let r = EmptyRange::new();
    let i = s.intersection(r);
    assert!(i.is_empty());
    assert_eq!(i.collect::<Vec<_>>(), vec![]);
}

#[test]
fn bounded_set() {
    let s1 = (1..2).union(3..);
    let s2 = (0..3).union(3..5);
    let i = s1.clone().intersection(s2);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![1, 3, 4]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    let s3 = (0..1).union(4..1);
    assert!(s1.intersection(s3).is_empty());
}

#[test]
fn lower_bounded_set() {
    let s1 = (1..2).union(3..);
    let s2 = (0..2).union(4..);
    let i = s1.intersection(s2);
    assert_eq!(i.clone().take(3).collect::<Vec<_>>(), vec![1, 4, 5]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(!i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.is_empty());
}

#[test]
fn upper_bounded_set() {
    let s1 = (1..2).union(3..);
    let s2 = (..3).union(4..5);
    println!("S1: {:?}", s1);
    println!("S2: {:?}", s2);
    let i = s1.clone().intersection(s2);
    println!("I: {:?}", i);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![1, 4]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(!i.contains(3));
    assert!(i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    let s2 = (..1).union(2..3);
    assert!(s1.intersection(s2).is_empty());
}

#[test]
fn unbounded_set() {
    let s = (1..2).union(3..);
    let r = (..3).union(3..4).union(5..);
    let i = s.intersection(r);
    assert_eq!(i.clone().take(3).collect::<Vec<_>>(), vec![1, 3, 5]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(!i.is_empty());
}
