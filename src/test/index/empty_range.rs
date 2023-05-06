use crate::EmptyRange;

#[test]
fn string() {
    let s: String = "hello, world!".to_string();
    let r: EmptyRange<usize> = EmptyRange::new();
    assert_eq!(&s[r], "");
}

#[test]
fn str() {
    let s: &str = "hello, world!";
    let r: EmptyRange<usize> = EmptyRange::new();
    assert_eq!(&s[r], "");
}

#[test]
fn slice() {
    let outer = vec![1, 2, 3, 4, 5];
    let slice: &[_] = &outer;
    let r = EmptyRange::new();
    assert_eq!(slice[r], []);
}

#[test]
fn vec() {
    let v: Vec<_> = vec![1, 2, 3, 4, 5];
    let r = EmptyRange::new();
    assert_eq!(v[r], vec![]);
}
