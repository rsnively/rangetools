use crate::EmptyRange;

#[test]
fn next() {
    let r = EmptyRange::<i32>::new();
    assert_eq!(r.into_iter().next(), None);
}

#[test]
fn size_hint() {
    let r = EmptyRange::<i32>::new();
    assert_eq!(r.into_iter().size_hint(), (0, Some(0)));
}

#[test]
fn count() {
    let r = EmptyRange::<i32>::new();
    assert_eq!(r.into_iter().count(), 0);
}

#[test]
fn last() {
    let r = EmptyRange::<i32>::new();
    assert_eq!(r.into_iter().last(), None);
}

#[test]
fn nth() {
    let r = EmptyRange::<i32>::new();
    assert_eq!(r.into_iter().nth(1), None);
}

#[test]
fn next_back() {
    let r = EmptyRange::<i32>::new();
    assert_eq!(r.into_iter().next_back(), None);
}

#[test]
fn nth_back() {
    let r = EmptyRange::<i32>::new();
    assert_eq!(r.into_iter().nth_back(2), None);
}
