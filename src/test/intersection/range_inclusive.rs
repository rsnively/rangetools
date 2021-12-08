use crate::Rangetools as _;

#[test]
fn range() {
    let i = (0..=3).intersection(2..5);
    assert_eq!(i.collect::<Vec<_>>(), vec![2, 3]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!((0..=3).intersection(10..20).is_empty());
}

#[test]
fn range_from() {
    let i = (0..=3).intersection(2..);
    assert_eq!(i.collect::<Vec<_>>(), vec![2, 3]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!((0..=3).intersection(10..).is_empty());
}

#[test]
fn range_full() {
    let i = (0..=3).intersection(..);
    assert_eq!(i.collect::<Vec<_>>(), vec![0, 1, 2, 3]);
    assert!(!i.contains(-1));
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
    assert!((2..=1).intersection(..).is_empty());
}

#[test]
fn range_inclusive() {
    let i = (0..=3).intersection(1..=2);
    assert_eq!(i.collect::<Vec<_>>(), vec![1, 2]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!((0..=3).intersection(10..=20).is_empty());
}

#[test]
fn range_to() {
    let i = (1..=3).intersection(..2);
    assert_eq!(i.collect::<Vec<_>>(), vec![1]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
    assert!((3..=5).intersection(..2).is_empty());
}

#[test]
fn range_to_inclusive() {
    let i = (1..=3).intersection(..=2);
    assert_eq!(i.collect::<Vec<_>>(), vec![1, 2]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
    assert!((3..=5).intersection(..=2).is_empty());
}

#[test]
fn bounded_range() {
    let br = (2..5).intersection(..);
    let i = (0..=3).intersection(br);
    assert_eq!(i.collect::<Vec<_>>(), vec![2, 3]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!((10..=20).intersection(br).is_empty());
}

#[test]
fn lower_bounded_range() {
    let br = (2..).intersection(..);
    let i = (0..=3).intersection(br);
    assert_eq!(i.collect::<Vec<_>>(), vec![2, 3]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!((0..=1).intersection(br).is_empty());
}

#[test]
fn upper_bounded_range() {
    let br = (..2).intersection(..);
    let i = (1..=3).intersection(br);
    assert_eq!(i.collect::<Vec<_>>(), vec![1]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
    assert!((3..=5).intersection(br).is_empty());
}

#[test]
fn unbounded_range() {
    let r = (..).intersection(..);
    let i = (0..=3).intersection(r);
    assert_eq!(i.collect::<Vec<_>>(), vec![0, 1, 2, 3]);
    assert!(!i.contains(-1));
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
    assert!((2..=1).intersection(r).is_empty());
}

#[test]
fn bounded_set() {
    let s = (1..3).union(4..6);
    let i = (2..=5).intersection(s.clone());
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![2, 4, 5]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!((10..=15).intersection(s).is_empty());
}

#[test]
fn lower_bounded_set() {
    let s = (1..2).union(3..);
    let i = (1..=4).intersection(s.clone());
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![1, 3, 4]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!((0..=0).intersection(s).is_empty());
}

#[test]
fn upper_bounded_set() {
    let s = (..2).union(3..5);
    let i = (1..=3).intersection(s.clone());
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![1, 3]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!((5..=5).intersection(s).is_empty());
}

#[test]
fn unbounded_set() {
    let s = (..2).union(3..4).union(5..);
    let i = (1..=5).intersection(s.clone());
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![1, 3, 5]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!((2..=2).intersection(s).is_empty());
}
