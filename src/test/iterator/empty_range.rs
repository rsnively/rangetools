use crate::EmptyRange;

#[test]
fn test_next() {
    let mut r = EmptyRange::<i32>::new();
    assert_eq!(r.next(), None);
}
