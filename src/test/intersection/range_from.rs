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
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(!i.contains(2));
    assert!(i.contains(3));
    assert!(i.contains(4));
    assert!(i.contains(5));
    assert!(!i.is_empty());
}
