use crate::Rangetools as _;

#[test]
fn range() {
    let r = (..).intersection(..);
    let i = r.intersection(1..3);
    assert_eq!(i.collect::<Vec<_>>(), vec![1, 2]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
    assert!(r.intersection(1..1).is_empty());
}

#[test]
fn range_from() {
    let r = (..).intersection(..);
    let i = r.intersection(3..);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(!i.is_empty());
}

#[test]
fn range_full() {
    let r = (..).intersection(..);
    let i = r.intersection(..);
    assert!(i.contains(0));
    assert!(i.contains(100));
    assert!(i.contains(1000));
    assert!(!i.is_empty());
}

#[test]
fn range_inclusive() {
    let r = (..).intersection(..);
    let i = r.intersection(1..=3);
    assert_eq!(i.collect::<Vec<_>>(), vec![1, 2, 3]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
    assert!(r.intersection(1..=0).is_empty());
}

#[test]
fn range_to() {
    let r = (..).intersection(..);
    let i = r.intersection(..3);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
}

#[test]
fn range_to_inclusive() {
    let r = (..).intersection(..);
    let i = r.intersection(..=3);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
}

#[test]
fn bounded_range() {
    let r1 = (..).intersection(..);
    let r2 = (1..3).intersection(..);
    let i = r1.intersection(r2);
    assert_eq!(i.collect::<Vec<_>>(), vec![1, 2]);
    assert!(!i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
    let r3 = (1..3).intersection(5..10);
    assert!(r1.intersection(r3).is_empty());
}

#[test]
fn lower_bounded_range() {
    let r1 = (..).intersection(..);
    let r2 = (3..).intersection(..);
    let i = r1.intersection(r2);
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
    let r1 = (..).intersection(..);
    let r2 = (..3).intersection(..);
    let i = r1.intersection(r2);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.is_empty());
}
#[test]
fn unbounded_range() {
    let r1 = (..).intersection(..);
    let r2 = (..).intersection(..);
    let i = r1.intersection(r2);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(!i.is_empty());
}
