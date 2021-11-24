use crate::RangeIntersection as _;

#[test]
fn range() {
    let i = (0..3).intersection(1..5);
    assert_eq!(i.collect::<Vec<_>>(), vec![1, 2]);
    assert!(!i.contains(-1));
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!((0..10).intersection(10..20).is_empty());
}

#[test]
fn range_from() {
    let i = (0..5).intersection(3..);
    assert_eq!(i.collect::<Vec<_>>(), vec![3, 4]);
    assert!(!i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!((0..3).intersection(3..).is_empty());
}

#[test]
fn range_full() {
    let i = (0..5).intersection(..);
    assert_eq!(i.collect::<Vec<_>>(), vec![0, 1, 2, 3, 4]);
    assert!(!i.contains(-1));
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!(!(0..3).intersection(..).is_empty());
}

#[test]
fn range_inclusive() {
    let i = (0..5).intersection(1..=3);
    assert_eq!(i.collect::<Vec<_>>(), vec![1, 2, 3]);
    assert!(!i.contains(0));
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.is_empty());
    assert!((0..3).intersection(3..=5).is_empty());
}

#[test]
fn range_to() {
    let i = (0..5).intersection(..3);
    assert_eq!(i.collect::<Vec<_>>(), vec![0, 1, 2]);
    assert!(!i.contains(-1));
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
    assert!((5..10).intersection(..5).is_empty());
}

#[test]
fn range_to_inclusive() {
    let i = (0..5).intersection(..3);
    assert_eq!(i.collect::<Vec<_>>(), vec![0, 1, 2]);
    assert!(!i.contains(-1));
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!(i.is_empty()));
    assert!((5..10).intersection(0..=4).is_empty());
}

#[test]
fn bounded_range() {
    let i = (0..3).intersection((1..5).intersection(2..4));
    assert_eq!(i.collect::<Vec<_>>(), vec![2]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    assert!((0..3)
        .intersection((10..20).intersection(15..17))
        .is_empty());
}

#[test]
fn lower_bounded_range() {
    let i = (0..3).intersection((1..).intersection(..));
    assert_eq!(i.collect::<Vec<_>>(), vec![1, 2]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    assert!((0..3).intersection((10..).intersection(..)).is_empty());
}

#[test]
fn upper_bounded_range() {
    let i = (0..3).intersection((..2).intersection(..));
    assert_eq!(i.collect::<Vec<_>>(), vec![0, 1]);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    assert!((5..10).intersection((..3).intersection(..)).is_empty());
}

#[test]
fn unbounded_range() {
    let i = (0..3).intersection((..).intersection(..));
    assert!(!i.contains(-1));
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
}
