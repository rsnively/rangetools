use crate::Rangetools as _;

#[test]
fn range() {
    let r = (1..).intersection(..);
    let i = r.intersection(2..4);
    assert_eq!(i.collect::<Vec<_>>(), vec![2, 3]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    assert!(r.intersection(0..1).is_empty());
}

#[test]
fn range_from() {
    let r = (1..).intersection(..);
    let i = r.intersection(2..);
    assert_eq!(i.take(3).collect::<Vec<_>>(), vec![2, 3, 4]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.is_empty());
}

#[test]
fn range_full() {
    let r = (1..).intersection(..);
    let i = r.intersection(..);
    assert_eq!(i.take(3).collect::<Vec<_>>(), vec![1, 2, 3]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.is_empty());
}

#[test]
fn range_inclusive() {
    let r = (1..).intersection(..);
    let i = r.intersection(2..=4);
    assert_eq!(i.collect::<Vec<_>>(), vec![2, 3, 4]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    assert!(r.intersection(0..=0).is_empty());
}

#[test]
fn range_to() {
    let r = (1..).intersection(..);
    let i = r.intersection(..4);
    assert_eq!(i.collect::<Vec<_>>(), vec![1, 2, 3]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    assert!(r.intersection(..1).is_empty());
}

#[test]
fn range_to_inclusive() {
    let r = (1..).intersection(..);
    let i = r.intersection(..=4);
    assert_eq!(i.collect::<Vec<_>>(), vec![1, 2, 3, 4]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    assert!(r.intersection(..=0).is_empty());
}

#[test]
fn bounded_range() {
    let r1 = (1..).intersection(..);
    let r2 = (2..5).intersection(..);
    let i = r1.intersection(r2);
    assert_eq!(i.collect::<Vec<_>>(), vec![2, 3, 4]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    let r3 = (0..1).intersection(..);
    assert!(r1.intersection(r3).is_empty());
}

#[test]
fn lower_bounded_range() {
    let r1 = (1..).intersection(..);
    let r2 = (2..).intersection(..);
    let i = r1.intersection(r2);
    assert_eq!(i.take(3).collect::<Vec<_>>(), vec![2, 3, 4]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.is_empty());
}

#[test]
fn upper_bounded_range() {
    let r1 = (1..).intersection(..);
    let r2 = (..5).intersection(..);
    let i = r1.intersection(r2);
    assert_eq!(i.collect::<Vec<_>>(), vec![1, 2, 3, 4]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    let r3 = (..1).intersection(..);
    assert!(r1.intersection(r3).is_empty());
}

#[test]
fn unbounded_range() {
    let r1 = (1..).intersection(..);
    let r2 = (..).intersection(..);
    let i = r1.intersection(r2);
    assert_eq!(i.take(3).collect::<Vec<_>>(), vec![1, 2, 3]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.is_empty());
}

#[test]
fn bounded_set() {
    let r = (1..).intersection(2..);
    let s = (2..3).union(4..5);
    let i = r.intersection(s);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![2, 4]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    let s = (0..1).union(0..1);
    assert!(r.intersection(s).is_empty());
}

#[test]
fn lower_bounded_set() {
    let r = (1..).intersection(2..);
    let s = (0..2).union(4..);
    let i = r.intersection(s);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(!i.contains(2));
    assert!(!i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.is_empty());
}

#[test]
fn upper_bounded_set() {
    let r = (1..).to_inner();
    let s = (..2).union(3..4);
    let i = r.intersection(s);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![1, 3]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    let s = (..1).union(..1);
    assert!(r.intersection(s).is_empty());
}

#[test]
fn unbounded_set() {
    let r = (1..).intersection(2..);
    let s = (..1).union(2..3).union(4..);
    let i = r.intersection(s);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.is_empty());
}
