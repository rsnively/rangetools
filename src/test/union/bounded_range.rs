use crate::{EmptyRange, Rangetools as _};

#[test]
fn range() {
    let r = (1..3).intersection(..);
    let u = r.union(5..7);
    assert_eq!(u.clone().into_iter().collect::<Vec<_>>(), vec![1, 2, 5, 6]);
    assert!(!u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(!u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.contains(7));
    assert!(!u.contains(8));
    assert!(!u.is_empty());
    let r2 = (1..1).intersection(..);
    assert!(r2.union(1..1).is_empty());
}

#[test]
fn range_from() {
    let r = (1..5).to_inner();
    let u = r.union(2..);
    assert_eq!(
        u.clone().into_iter().take(3).collect::<Vec<_>>(),
        vec![1, 2, 3]
    );
    assert!(!u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn range_full() {
    let r = (1..3).intersection(4..5);
    let u = r.union(..);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
    let empty_range = (1..3).intersection(4..5);
    assert!(!empty_range.union(..).is_empty());
}

#[test]
fn range_inclusive() {
    let r = (1..3).intersection(..);
    let u = r.union(5..=7);
    assert_eq!(
        u.clone().into_iter().collect::<Vec<_>>(),
        vec![1, 2, 5, 6, 7]
    );
    assert!(!u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(!u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(u.contains(7));
    assert!(!u.contains(8));
    assert!(!u.is_empty());
    let r2 = (1..1).intersection(..);
    assert!(r2.union(1..=0).is_empty());
}

#[test]
fn range_to() {
    let r = (1..3).intersection(..);
    let u = r.union(..2);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(!u.contains(4));
    assert!(!u.contains(5));
    assert!(!u.contains(6));
    assert!(!u.contains(7));
    assert!(!u.contains(8));
    assert!(!u.is_empty());
}

#[test]
fn range_to_inclusive() {
    let r = (1..3).intersection(..);
    let u = r.union(..=4);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(!u.contains(5));
    assert!(!u.contains(6));
    assert!(!u.contains(7));
    assert!(!u.contains(8));
    assert!(!u.is_empty());
}

#[test]
fn bounded_range() {
    let r1 = (1..3).intersection(..);
    let r2 = (5..7).intersection(..);
    let u = r1.union(r2);
    assert_eq!(u.clone().into_iter().collect::<Vec<_>>(), vec![1, 2, 5, 6]);
    assert!(!u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(!u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.contains(7));
    assert!(!u.contains(8));
    assert!(!u.is_empty());
    let r3 = (1..1).intersection(..);
    assert!(r3.union(1..1).is_empty());
}

#[test]
fn lower_bounded_range() {
    let r1 = (1..5).to_inner();
    let r2 = (..).intersection(2..);
    let u = r1.union(r2);
    assert_eq!(
        u.clone().into_iter().take(5).collect::<Vec<_>>(),
        vec![1, 2, 3, 4, 5]
    );
    assert!(!u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn upper_bounded_range() {
    let r1 = (1..3).intersection(..);
    let r2 = (..1).intersection(..);
    let u = r1.union(r2);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(!u.contains(4));
    assert!(!u.contains(5));
    assert!(!u.contains(6));
    assert!(!u.contains(7));
    assert!(!u.contains(8));
    assert!(!u.is_empty());
}

#[test]
fn unbounded_range() {
    let r = (1..3).intersection(..);
    let ur = (..).intersection(..);
    let u = r.union(ur);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
    let empty_range = (1..3).intersection(4..5);
    assert!(!empty_range.union(ur).is_empty());
}

#[test]
fn empty_range() {
    let r = (1..3).intersection(..);
    let u = r.union(EmptyRange::new());
    assert_eq!(r, u);
}

#[test]
fn bounded_set() {
    let r = (1..3).intersection(..);
    let s = (5..7).union(2..4);
    let u = r.union(s);
    assert_eq!(
        u.clone().into_iter().collect::<Vec<_>>(),
        vec![1, 2, 3, 5, 6]
    );
    assert!(!u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(!u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.contains(7));
    assert!(!u.contains(8));
    assert!(!u.is_empty());
    let empty_set = (1..1).union(2..2);
    let empty_range = (1..3).intersection(4..5);
    assert!(empty_range.union(empty_set).is_empty());
}

#[test]
fn lower_bounded_set() {
    let r = (1..5).to_inner();
    let s = (2..3).union(5..);
    let u = r.union(s);
    assert_eq!(
        u.clone().into_iter().take(5).collect::<Vec<_>>(),
        vec![1, 2, 3, 4, 5]
    );
    assert!(!u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(u.contains(3));
    assert!(u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}

#[test]
fn upper_bounded_set() {
    let r = (1..3).intersection(..);
    let s = (..1).union(4..5);
    let u = r.union(s);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(u.contains(4));
    assert!(!u.contains(5));
    assert!(!u.contains(6));
    assert!(!u.contains(7));
    assert!(!u.contains(8));
    assert!(!u.is_empty());
}

#[test]
fn unbounded_set() {
    let r = (1..3).intersection(..);
    let s = (..2).union(5..);
    let u = r.union(s);
    assert!(u.contains(0));
    assert!(u.contains(1));
    assert!(u.contains(2));
    assert!(!u.contains(3));
    assert!(!u.contains(4));
    assert!(u.contains(5));
    assert!(u.contains(6));
    assert!(!u.is_empty());
}
