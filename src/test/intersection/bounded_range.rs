use crate::{EmptyRange, Rangetools as _};

#[test]
fn range() {
    let bounded_range = (1..4).intersection(2..5);
    let i = bounded_range.intersection(3..6);
    assert_eq!(i.into_iter().collect::<Vec<_>>(), vec![3]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!(bounded_range.intersection(10..20).is_empty());
}

#[test]
fn range_from() {
    let bounded_range = (1..4).intersection(2..5);
    let i = bounded_range.intersection(3..);
    assert_eq!(i.into_iter().collect::<Vec<_>>(), vec![3]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!(bounded_range.intersection(10..).is_empty());
}

#[test]
fn range_full() {
    let bounded_range = (1..4).intersection(2..5);
    let i = bounded_range.intersection(..);
    assert_eq!(i.into_iter().collect::<Vec<_>>(), vec![2, 3]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!(((1..3).intersection(5..10)).intersection(..).is_empty());
}

#[test]
fn range_inclusive() {
    let bounded_range = (1..4).intersection(2..5);
    let i = bounded_range.intersection(1..=3);
    assert_eq!(i.into_iter().collect::<Vec<_>>(), vec![2, 3]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!(bounded_range.intersection(10..=20).is_empty());
}

#[test]
fn range_to() {
    let bounded_range = (1..4).intersection(2..5);
    let i = bounded_range.intersection(..3);
    assert_eq!(i.into_iter().collect::<Vec<_>>(), vec![2]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!(bounded_range.intersection(..1).is_empty());
}

#[test]
fn range_to_inclusive() {
    let bounded_range = (1..4).intersection(2..5);
    let i = bounded_range.intersection(..=3);
    assert_eq!(i.into_iter().collect::<Vec<_>>(), vec![2, 3]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!(bounded_range.intersection(..=1).is_empty());
}

#[test]
fn bounded_range() {
    let r1 = (0..5).intersection(1..5);
    let r2 = (2..6).intersection(2..7);
    let i = r1.intersection(r2);
    assert_eq!(i.into_iter().collect::<Vec<_>>(), vec![2, 3, 4]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.contains(7));
    assert!(!i.contains(8));
    assert!(!i.is_empty());
    let r3 = (10..20).intersection(15..20);
    assert!(i.intersection(r3).is_empty());
}

#[test]
fn lower_bounded_range() {
    let r1 = (1..3).intersection(..);
    let r2 = (1..).intersection(..);
    let i = r1.intersection(r2);
    assert_eq!(i.into_iter().collect::<Vec<_>>(), vec![1, 2]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
    let r3 = (5..).intersection(..);
    assert!(i.intersection(r3).is_empty());
}

#[test]
fn upper_bounded_range() {
    let r1 = (1..3).intersection(..);
    let r2 = (..3).intersection(..);
    let i = r1.intersection(r2);
    assert_eq!(i.into_iter().collect::<Vec<_>>(), vec![1, 2]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
    let r3 = (..1).intersection(..);
    assert!(i.intersection(r3).is_empty());
}

#[test]
fn unbounded_range() {
    let r1 = (1..3).intersection(..);
    let r2 = (..).intersection(..);
    let i = r1.intersection(r2);
    assert_eq!(i.into_iter().collect::<Vec<_>>(), vec![1, 2]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
    let r3 = (1..3).intersection(5..10);
    assert!(r3.intersection(..).is_empty());
}

#[test]
fn empty_range() {
    let r1 = (1..4).intersection(..);
    let r2 = EmptyRange::new();
    let i = r1.intersection(r2);
    assert!(i.is_empty());
    assert_eq!(i.into_iter().collect::<Vec<_>>(), vec![]);
}

#[test]
fn bounded_set() {
    let r = (1..4).intersection(..);
    let s = (1..2).union(3..4);
    let i = r.intersection(s);
    assert_eq!(i.clone().into_iter().collect::<Vec<_>>(), vec![1, 3]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
    let r2 = (1..1).intersection(..);
    let s2 = (1..2).union(3..4);
    assert!(r2.intersection(s2).is_empty());
}

#[test]
fn lower_bounded_set() {
    let r = (1..4).intersection(..);
    let s = (1..2).union(3..);
    let i = r.intersection(s);
    assert_eq!(i.clone().into_iter().collect::<Vec<_>>(), vec![1, 3]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
    let r2 = (2..3).intersection(..);
    let s2 = (1..2).union(3..);
    assert!(r2.intersection(s2).is_empty());
}

#[test]
fn upper_bounded_set() {
    let r = (1..4).intersection(..);
    let s = (..2).union(3..4);
    let i = r.intersection(s);
    assert_eq!(i.clone().into_iter().collect::<Vec<_>>(), vec![1, 3]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
    let r2 = (1..1).intersection(..);
    let s2 = (..2).union(3..4);
    assert!(r2.intersection(s2).is_empty());
}

#[test]
fn unbounded_set() {
    let r = (1..4).to_inner();
    let s = (..1).union(2..3).union(4..);
    let i = r.intersection(s);
    assert_eq!(i.clone().into_iter().collect::<Vec<_>>(), vec![2]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
    let r2 = (1..1).to_inner();
    let s2 = (..1).union(2..3).union(4..);
    assert!(r2.intersection(s2).is_empty());
}
