use crate::{BoundedRange, LowerBound, RangeIntersection, UpperBound};

#[test]
fn next_included_included() {
    let r: BoundedRange<i32> = (0..=3).into();
    let mut it = r.into_iter();
    assert_eq!(it.next(), Some(0));
    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next(), Some(3));
    assert_eq!(it.next(), None);
}

#[test]
fn next_included_excluded() {
    let r: BoundedRange<i32> = (0..3).into();
    let mut it = r.into_iter();
    assert_eq!(it.next(), Some(0));
    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next(), None);
}

#[test]
fn next_excluded_included() {
    let r = BoundedRange::new(LowerBound::excluded(3), UpperBound::included(5));
    let mut it = r.into_iter();
    assert_eq!(it.next(), Some(4));
    assert_eq!(it.next(), Some(5));
    assert_eq!(it.next(), None);
}

#[test]
fn next_excluded_excluded() {
    let r = BoundedRange::new(LowerBound::excluded(3), UpperBound::excluded(5));
    let mut it = r.into_iter();
    assert_eq!(it.next(), Some(4));
    assert_eq!(it.next(), None);
}
#[test]
fn size_hint() {
    let r1 = BoundedRange::new(LowerBound::excluded(5), UpperBound::excluded(10));
    assert_eq!(r1.into_iter().size_hint(), (4, Some(4)));

    let r2 = BoundedRange::new(LowerBound::included(5), UpperBound::excluded(10));
    assert_eq!(r2.into_iter().size_hint(), (5, Some(5)));

    let r3 = BoundedRange::new(LowerBound::excluded(5), UpperBound::included(10));
    assert_eq!(r3.into_iter().size_hint(), (5, Some(5)));

    let r4 = BoundedRange::new(LowerBound::included(5), UpperBound::included(10));
    assert_eq!(r4.into_iter().size_hint(), (6, Some(6)));

    let r5: BoundedRange<i32> = (0..10).intersection(10..20);
    assert_eq!(r5.into_iter().size_hint(), (0, Some(0)));
}

#[test]
fn count() {
    let r1 = BoundedRange::new(LowerBound::excluded(5), UpperBound::excluded(10));
    assert_eq!(r1.into_iter().count(), 4);

    let r2 = BoundedRange::new(LowerBound::included(5), UpperBound::excluded(10));
    assert_eq!(r2.into_iter().count(), 5);

    let r3 = BoundedRange::new(LowerBound::excluded(5), UpperBound::included(10));
    assert_eq!(r3.into_iter().count(), 5);

    let r4 = BoundedRange::new(LowerBound::included(5), UpperBound::included(10));
    assert_eq!(r4.into_iter().count(), 6);

    let r5: BoundedRange<i32> = (0..10).intersection(10..20);
    assert_eq!(r5.into_iter().count(), 0);
}

#[test]
fn last() {
    let r1 = BoundedRange::new(LowerBound::excluded(5), UpperBound::excluded(10));
    assert_eq!(r1.into_iter().last(), Some(9));

    let r2 = BoundedRange::new(LowerBound::included(5), UpperBound::excluded(10));
    assert_eq!(r2.into_iter().last(), Some(9));

    let r3 = BoundedRange::new(LowerBound::excluded(5), UpperBound::included(10));
    assert_eq!(r3.into_iter().last(), Some(10));

    let r4 = BoundedRange::new(LowerBound::included(5), UpperBound::included(10));
    assert_eq!(r4.into_iter().last(), Some(10));

    let r5: BoundedRange<i32> = (0..10).intersection(10..20);
    assert_eq!(r5.into_iter().last(), None);
}

#[test]
fn nth() {
    let r1 = BoundedRange::new(LowerBound::excluded(5), UpperBound::excluded(10));
    assert_eq!(r1.into_iter().nth(1), Some(7));

    let r2 = BoundedRange::new(LowerBound::included(5), UpperBound::excluded(10));
    assert_eq!(r2.into_iter().nth(1), Some(6));

    let r3 = BoundedRange::new(LowerBound::excluded(5), UpperBound::included(10));
    assert_eq!(r3.into_iter().nth(10), None);

    let r4 = BoundedRange::new(LowerBound::included(5), UpperBound::included(10));
    assert_eq!(r4.into_iter().nth(10), None);

    let r5: BoundedRange<i32> = (0..10).intersection(10..20);
    assert_eq!(r5.into_iter().nth(0), None);
}

#[test]
fn min() {
    let r1 = BoundedRange::new(LowerBound::excluded(5), UpperBound::excluded(10));
    assert_eq!(r1.into_iter().min(), Some(6));

    let r2 = BoundedRange::new(LowerBound::included(5), UpperBound::excluded(10));
    assert_eq!(r2.into_iter().min(), Some(5));

    let r3 = BoundedRange::new(LowerBound::excluded(5), UpperBound::included(10));
    assert_eq!(r3.into_iter().min(), Some(6));

    let r4 = BoundedRange::new(LowerBound::included(5), UpperBound::included(10));
    assert_eq!(r4.into_iter().min(), Some(5));

    let r5: BoundedRange<i32> = (0..10).intersection(10..20);
    assert_eq!(r5.into_iter().min(), None);
}

#[test]
fn max() {
    let r1 = BoundedRange::new(LowerBound::excluded(5), UpperBound::excluded(10));
    assert_eq!(r1.into_iter().max(), Some(9));

    let r2 = BoundedRange::new(LowerBound::included(5), UpperBound::excluded(10));
    assert_eq!(r2.into_iter().max(), Some(9));

    let r3 = BoundedRange::new(LowerBound::excluded(5), UpperBound::included(10));
    assert_eq!(r3.into_iter().max(), Some(10));

    let r4 = BoundedRange::new(LowerBound::included(5), UpperBound::included(10));
    assert_eq!(r4.into_iter().max(), Some(10));

    let r5: BoundedRange<i32> = (0..10).intersection(10..20);
    assert_eq!(r5.into_iter().max(), None);
}

#[test]
fn next_back() {
    let r1 = BoundedRange::new(LowerBound::excluded(5), UpperBound::excluded(7));
    let mut i1 = r1.into_iter();
    assert_eq!(i1.next_back(), Some(6));
    assert_eq!(i1.next_back(), None);

    let r2 = BoundedRange::new(LowerBound::included(5), UpperBound::excluded(7));
    let mut i2 = r2.into_iter();
    assert_eq!(i2.next_back(), Some(6));
    assert_eq!(i2.next_back(), Some(5));
    assert_eq!(i2.next_back(), None);

    let r3 = BoundedRange::new(LowerBound::excluded(5), UpperBound::included(7));
    let mut i3 = r3.into_iter();
    assert_eq!(i3.next_back(), Some(7));
    assert_eq!(i3.next_back(), Some(6));
    assert_eq!(i3.next_back(), None);

    let r4 = BoundedRange::new(LowerBound::included(5), UpperBound::included(7));
    let mut i4 = r4.into_iter();
    assert_eq!(i4.next_back(), Some(7));
    assert_eq!(i4.next_back(), Some(6));
    assert_eq!(i4.next_back(), Some(5));
    assert_eq!(i4.next_back(), None);

    let r5: BoundedRange<i32> = (0..10).intersection(10..20);
    assert_eq!(r5.into_iter().next_back(), None);
}

#[test]
fn nth_back() {
    let r1 = BoundedRange::new(LowerBound::excluded(5), UpperBound::excluded(10));
    assert_eq!(r1.into_iter().nth_back(1), Some(8));

    let r2 = BoundedRange::new(LowerBound::included(5), UpperBound::excluded(10));
    assert_eq!(r2.into_iter().nth_back(2), Some(7));

    let r3 = BoundedRange::new(LowerBound::excluded(5), UpperBound::included(10));
    assert_eq!(r3.into_iter().nth_back(10), None);

    let r4 = BoundedRange::new(LowerBound::included(5), UpperBound::included(10));
    assert_eq!(r4.into_iter().nth_back(10), None);

    let r5: BoundedRange<i32> = (0..10).intersection(10..20);
    assert_eq!(r5.into_iter().nth_back(0), None);
}
