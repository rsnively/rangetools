use crate::{
    BoundedRange, BoundedSet, EmptyRange, LowerBound, LowerBoundedRange, LowerBoundedSet,
    Rangetools as _, UnboundedRange, UnboundedSet, UpperBound, UpperBoundedRange, UpperBoundedSet,
};

#[test]
fn range() {
    let r: std::ops::Range<i32> = 5..10;
    let c = UnboundedSet::new(
        UpperBoundedRange::new(UpperBound::excluded(5)),
        LowerBoundedRange::new(LowerBound::included(10)),
    );
    assert_eq!(r.complement(), c);
}

#[test]
fn range_inclusive() {
    let r: std::ops::RangeInclusive<i32> = 5..=10;
    let c = UnboundedSet::new(
        UpperBoundedRange::new(UpperBound::excluded(5)),
        LowerBoundedRange::new(LowerBound::excluded(10)),
    );
    assert_eq!(r.complement(), c);
}

#[test]
fn range_to() {
    let r: std::ops::RangeTo<i32> = ..4;
    let c = LowerBoundedRange::new(LowerBound::included(4));
    assert_eq!(r.complement(), c);
}

#[test]
fn range_to_inclusive() {
    let r: std::ops::RangeToInclusive<i32> = ..=4;
    let c = LowerBoundedRange::new(LowerBound::excluded(4));
    assert_eq!(r.complement(), c);
}

#[test]
fn range_from() {
    let r: std::ops::RangeFrom<i32> = 2..;
    let c = UpperBoundedRange::new(UpperBound::excluded(2));
    assert_eq!(r.complement(), c);
}

#[test]
fn range_full() {
    let r: std::ops::RangeFull = ..;
    let c = EmptyRange::<i32>::new();
    assert_eq!(r.complement(), c);
}

#[test]
fn bounded_range() {
    let r: BoundedRange<i32> = (1..3).to_inner();
    let c = UnboundedSet::new(
        UpperBoundedRange::new(UpperBound::excluded(1)),
        LowerBoundedRange::new(LowerBound::included(3)),
    );
    assert_eq!(r.complement(), c);
}

#[test]
fn lower_bounded_range() {
    let r: LowerBoundedRange<i32> = (1..).to_inner();
    let c = UpperBoundedRange::new(UpperBound::excluded(1));
    assert_eq!(r.complement(), c);
}

#[test]
fn upper_bounded_range() {
    let r: UpperBoundedRange<i32> = (..1).to_inner();
    let c = LowerBoundedRange::new(LowerBound::included(1));
    assert_eq!(r.complement(), c);
}

#[test]
fn unbounded_range() {
    let r: UnboundedRange = (..).to_inner();
    let c = EmptyRange::<i32>::new();
    assert_eq!(r.complement(), c);
}

#[test]
fn empty_range() {
    let r: EmptyRange<i32> = EmptyRange::new();
    let c = UnboundedRange::new();
    assert_eq!(r.complement(), c);
}

#[test]
fn bounded_set() {
    let s: BoundedSet<i32> = (1..3).union(5..10);
    let c = (..1).union(3..5).union(10..);
    assert_eq!(s.complement(), c);
}

#[test]
fn lower_bounded_set() {
    let s: LowerBoundedSet<i32> = (1..2).union(4..);
    let c = (..1).union(2..4);
    assert_eq!(s.complement(), c);
}

#[test]
fn upper_bounded_set() {
    let s: UpperBoundedSet<i32> = (..4).union(5..10);
    let c = (4..5).union(10..);
    assert_eq!(s.complement(), c);
}

#[test]
fn unbounded_set() {
    let s: UnboundedSet<i32> = (..1).union(3..5).union(10..);
    let c = (1..3).union(5..10);
    assert_eq!(s.complement(), c);
}
