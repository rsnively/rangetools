use crate::Rangetools as _;

#[test]
fn range() {
    let i = (3..).intersection(1..5);
    assert_eq!(i.collect::<Vec<_>>(), vec![3, 4]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    assert!((3..).intersection(1..1).is_empty());
}

#[test]
fn range_from() {
    let i = (3..).intersection(5..);
    assert_eq!(i.take(3).collect::<Vec<_>>(), vec![5, 6, 7]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(!i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(i.contains(6));
    assert!(!i.is_empty());
}

#[test]
fn range_full() {
    let i = (3..).intersection(..);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.is_empty());
}

#[test]
fn range_inclusive() {
    let i = (3..).intersection(1..=5);
    assert_eq!(i.collect::<Vec<_>>(), vec![3, 4, 5]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!((3..).intersection(1..=0).is_empty());
}

#[test]
fn range_to() {
    let i = (3..).intersection(..5);
    assert_eq!(i.collect::<Vec<_>>(), vec![3, 4]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!((10..).intersection(..5).is_empty());
}

#[test]
fn range_to_inclusive() {
    let i = (3..).intersection(..=5);
    assert_eq!(i.collect::<Vec<_>>(), vec![3, 4, 5]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!((10..).intersection(..=5).is_empty());
}

#[test]
fn bounded_range() {
    let br = (2..5).intersection(..);
    let i = (1..).intersection(br);
    assert_eq!(i.collect::<Vec<_>>(), vec![2, 3, 4]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!((10..).intersection(br).is_empty());
}

#[test]
fn lower_bounded_range() {
    let br = (2..).intersection(..);
    let i = (3..).intersection(br);
    assert_eq!(i.take(3).collect::<Vec<_>>(), vec![3, 4, 5]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(i.contains(6));
    assert!(!i.is_empty());
}
#[test]
fn upper_bounded_range() {
    let r = (..5).intersection(..);
    let i = (3..).intersection(r);
    assert_eq!(i.collect::<Vec<_>>(), vec![3, 4]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    let r2 = (..1).intersection(..);
    assert!((3..).intersection(r2).is_empty());
}
#[test]
fn unbounded_range() {
    let r = (..).intersection(..);
    let i = (3..).intersection(r);
    assert_eq!(i.take(3).collect::<Vec<_>>(), vec![3, 4, 5]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.is_empty());
}

#[test]
fn bounded_set() {
    let bs = (2..3).union(4..5);
    let i = (1..).intersection(bs.clone());
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![2, 4]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!((10..).intersection(bs).is_empty());
}

#[test]
fn lower_bounded_set() {
    let bs = (2..).union(0..1);
    let i = (3..).intersection(bs);
    assert_eq!(i.clone().take(3).collect::<Vec<_>>(), vec![3, 4, 5]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(i.contains(6));
    assert!(!i.is_empty());
}

#[test]
fn upper_bounded_set() {
    let s = (..3).union(4..5);
    let i = (3..).intersection(s);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![4]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(!i.contains(2));
    assert!(!i.contains(3));
    assert!(i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    let s2 = (..1).union(2..3);
    assert!((3..).intersection(s2).is_empty());
}

#[test]
fn unbounded_set() {
    let s = (5..).union(2..4).union(..1);
    let i = (3..).intersection(s);
    assert_eq!(i.clone().take(3).collect::<Vec<_>>(), vec![3, 5, 6]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(!i.is_empty());
}
