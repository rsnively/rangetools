use crate::BoundedRange;

#[test]
fn string() {
    let s: String = "hello, world!".to_string();
    let r1: BoundedRange<usize> = (0..5).into();
    assert_eq!(&s[r1], "hello");

    let r2: BoundedRange<usize> = (7..=11).into();
    assert_eq!(&s[r2], "world");
}

#[test]
fn string_mut() {
    let mut s: String = "hello, world!".to_string();
    let r: BoundedRange<usize> = (0..5).into();
    unsafe {
        let bytes = s[r].as_bytes_mut();
        bytes[2] = b'n';
    }
    assert_eq!(s, "henlo, world!".to_string());
}

#[test]
fn str() {
    let s: &str = "hello, world!";
    let r1: BoundedRange<usize> = (0..5).into();
    assert_eq!(&s[r1], "hello");

    let r2: BoundedRange<usize> = (7..=11).into();
    assert_eq!(&s[r2], "world");
}

#[test]
fn str_mut() {
    let mut outer: String = "hello, world!".to_string();
    let s: &mut str = outer.as_mut_str();
    let r: BoundedRange<usize> = (0..5).into();
    unsafe {
        let bytes = s[r].as_bytes_mut();
        bytes[2] = b'n';
    }
    assert_eq!(s, "henlo, world!");
}

#[test]
fn slice() {
    let outer = vec![1, 2, 3, 4, 5];
    let slice: &[_] = &outer;
    let r1: BoundedRange<_> = (0..2).into();
    assert_eq!(slice[r1], [1, 2]);

    let r2: BoundedRange<_> = (2..=4).into();
    assert_eq!(slice[r2], [3, 4, 5]);
}

#[test]
fn slice_mut() {
    let mut outer = vec![1, 2, 3, 4, 5];
    let slice: &mut [_] = &mut outer;
    let r: BoundedRange<_> = (1..3).into();
    *(slice[r].get_mut(0).unwrap()) = 7;
    assert_eq!(slice, [1, 7, 3, 4, 5]);
}

#[test]
fn vec() {
    let v: Vec<_> = vec![1, 2, 3, 4, 5];
    let r1: BoundedRange<_> = (0..2).into();
    assert_eq!(v[r1], vec![1, 2]);

    let r2: BoundedRange<_> = (2..=4).into();
    assert_eq!(v[r2], vec![3, 4, 5]);
}

#[test]
fn vec_mut() {
    let mut v: Vec<_> = vec![1, 2, 3, 4, 5];
    let r: BoundedRange<_> = (1..3).into();
    *(v[r].get_mut(0).unwrap()) = 7;
    assert_eq!(v, vec![1, 7, 3, 4, 5]);
}
