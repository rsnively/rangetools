use crate::Rangetools as _;

#[test]
fn range() {
    let i = (..=3).intersection(2..4);
    assert_eq!(i.collect::<Vec<_>>(), vec![2, 3]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    assert!((..=3).intersection(5..10).is_empty());
}

#[test]
fn range_from() {
    let i = (..=3).intersection(2..);
    assert_eq!(i.collect::<Vec<_>>(), vec![2, 3]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    assert!((..=3).intersection(5..).is_empty());
}

#[test]
fn range_full() {
    let i = (..=3).intersection(..);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
}

#[test]
fn range_inclusive() {
    let i = (..=3).intersection(2..=4);
    assert_eq!(i.collect::<Vec<_>>(), vec![2, 3]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    assert!((..=3).intersection(5..=10).is_empty());
}

#[test]
fn range_to() {
    let i = (..=3).intersection(..2);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
}

#[test]
fn range_to_inclusive() {
    let i = (..=3).intersection(..=2);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
}

#[test]
fn bounded_range() {
    let r = (2..4).intersection(..);
    let i = (..=3).intersection(r);
    assert_eq!(i.collect::<Vec<_>>(), vec![2, 3]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    let r2 = (5..10).intersection(..);
    assert!((..=3).intersection(r2).is_empty());
}

#[test]
fn lower_bounded_range() {
    let r = (2..).intersection(..);
    let i = (..=3).intersection(r);
    assert_eq!(i.collect::<Vec<_>>(), vec![2, 3]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
    let r2 = (5..).intersection(..);
    assert!((..=3).intersection(r2).is_empty());
}

#[test]
fn upper_bounded_range() {
    let r = (..2).intersection(..);
    let i = (..=3).intersection(r);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
}

#[test]
fn unbounded_range() {
    let r = (..).intersection(..);
    let i = (..=3).intersection(r);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(!i.is_empty());
}
