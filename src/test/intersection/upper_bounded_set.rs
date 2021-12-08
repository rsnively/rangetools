use crate::Rangetools as _;

#[test]
fn range() {
    let s = (..3).union(5..7);
    let i = s.clone().intersection(2..6);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![2, 5]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.contains(7));
    assert!(!i.contains(8));
    assert!(!i.is_empty());
    assert!(s.intersection(10..20).is_empty());
}

#[test]
fn range_from() {
    let s = (..3).union(5..7);
    let i = s.clone().intersection(2..);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![2, 5, 6]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(i.contains(6));
    assert!(!i.contains(7));
    assert!(!i.contains(8));
    assert!(!i.is_empty());
    assert!(s.intersection(10..).is_empty());
}

#[test]
fn range_full() {
    let s = (..3).union(5..7);
    let i = s.intersection(..);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(i.contains(6));
    assert!(!i.contains(7));
    assert!(!i.contains(8));
    assert!(!i.is_empty());
}

#[test]
fn range_inclusive() {
    let s = (..3).union(5..7);
    let i = s.clone().intersection(2..=6);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![2, 5, 6]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(i.contains(6));
    assert!(!i.contains(7));
    assert!(!i.contains(8));
    assert!(!i.is_empty());
    assert!(s.intersection(10..=20).is_empty());
}

#[test]
fn range_to() {
    let s = (..3).union(5..7);
    let i = s.intersection(..6);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.contains(7));
    assert!(!i.contains(8));
    assert!(!i.is_empty());
}

#[test]
fn range_to_inclusive() {
    let s = (..3).union(5..7);
    let i = s.intersection(..=6);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(i.contains(6));
    assert!(!i.contains(7));
    assert!(!i.contains(8));
    assert!(!i.is_empty());
}

#[test]
fn bounded_range() {
    let s = (..3).union(5..7);
    let r = (2..=6).intersection(..);
    let i = s.clone().intersection(r);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![2, 5, 6]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(i.contains(6));
    assert!(!i.contains(7));
    assert!(!i.contains(8));
    assert!(!i.is_empty());
    let r = (10..20).to_inner();
    assert!(s.intersection(r).is_empty());
}

#[test]
fn lower_bounded_range() {
    let s = (..3).union(5..7);
    let r = (2..).intersection(..);
    let i = s.clone().intersection(r);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![2, 5, 6]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(i.contains(6));
    assert!(!i.contains(7));
    assert!(!i.contains(8));
    assert!(!i.is_empty());
    let r = (10..).intersection(..);
    assert!(s.intersection(r).is_empty());
}

#[test]
fn upper_bounded_range() {
    let s = (..3).union(5..7);
    let r = (..6).intersection(..);
    let i = s.intersection(r);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.contains(7));
    assert!(!i.contains(8));
    assert!(!i.is_empty());
}

#[test]
fn unbounded_range() {
    let s = (..3).union(5..7);
    let r = (..).to_inner();
    let i = s.intersection(r);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(i.contains(6));
    assert!(!i.contains(7));
    assert!(!i.contains(8));
    assert!(!i.is_empty());
}

#[test]
fn bounded_set() {
    let s1 = (..3).union(5..7);
    let s2 = (2..5).union(6..9);
    let i = s1.clone().intersection(s2);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![2, 6]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(i.contains(6));
    assert!(!i.contains(7));
    assert!(!i.contains(8));
    assert!(!i.is_empty());
    let s2 = (10..20).union(30..40);
    assert!(s1.intersection(s2).is_empty());
}

#[test]
fn lower_bounded_set() {
    let s = (..3).union(5..7);
    let s2 = (2..5).union(6..);
    let i = s.clone().intersection(s2);
    assert_eq!(i.clone().collect::<Vec<_>>(), vec![2, 6]);
    assert!(!i.contains(0));
    assert!(!i.contains(1));
    assert!(i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(!i.contains(5));
    assert!(i.contains(6));
    assert!(!i.contains(7));
    assert!(!i.contains(8));
    assert!(!i.is_empty());
    let s2 = (10..).union(7..8);
    assert!(s.intersection(s2).is_empty());
}

#[test]
fn upper_bounded_set() {
    let s = (..3).union(5..7);
    let s2 = (..2).union(4..6);
    let i = s.intersection(s2);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.contains(7));
    assert!(!i.contains(8));
    assert!(!i.is_empty());
}

#[test]
fn unbounded_set() {
    let s = (..3).union(5..7);
    let s2 = (..2).union(4..6).union(8..);
    let i = s.intersection(s2);
    assert!(i.contains(0));
    assert!(i.contains(1));
    assert!(!i.contains(2));
    assert!(!i.contains(3));
    assert!(!i.contains(4));
    assert!(i.contains(5));
    assert!(!i.contains(6));
    assert!(!i.contains(7));
    assert!(!i.contains(8));
    assert!(!i.is_empty());
}
