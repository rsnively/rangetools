use crate::RangeIntersection as _;

#[test]
fn range() {
    let i = (..).intersection(1..3);
    assert_eq!(i.collect::<Vec<_>>(), vec![1, 2]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
    assert!((..).intersection(1..1).is_empty());
}

#[test]
fn range_from() {
    let i = (..).intersection(3..);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(!i.is_empty());
}

#[test]
fn range_full() {
    let i = (..).intersection(..);
    assert!(i.contains(0));
    assert!(i.contains(100));
    assert!(i.contains(1000));
    assert!(!i.is_empty());
}

#[test]
fn range_inclusive() {
    let i = (..).intersection(1..=3);
    assert_eq!(i.collect::<Vec<_>>(), vec![1, 2, 3]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
    assert!((..).intersection(1..=0).is_empty());
}

#[test]
fn range_to() {
    let i = (..).intersection(..3);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
}

#[test]
fn range_to_inclusive() {
    let i = (..).intersection(..=3);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
}

#[test]
fn bounded_range() {
    let r = (1..3).intersection(..);
    let i = (..).intersection(r);
    assert_eq!(i.collect::<Vec<_>>(), vec![1, 2]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
    let r2 = (1..3).intersection(5..10);
    assert!((..).intersection(r2).is_empty());
}

#[test]
fn lower_bounded_range() {
    let r = (3..).intersection(..);
    let i = (..).intersection(r);
    assert_eq!(i.take(3).collect::<Vec<_>>(), vec![3, 4, 5]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(!i.is_empty());
}
#[test]
fn upper_bounded_range() {
    let r = (..3).intersection(..);
    let i = (..).intersection(r);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
}
#[test]
fn unbounded_range() {
    let r = (..).intersection(..);
    let i = (..).intersection(r);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(!i.is_empty());
}
