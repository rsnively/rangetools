use crate::EmptyRange;

#[test]
fn test_next() {
    let r = EmptyRange::<i32>::new();
    assert_eq!(r.into_iter().next(), None);
}
