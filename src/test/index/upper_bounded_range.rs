use crate::UpperBoundedRange;

#[test]
fn string() {
    let s: String = "hello, world!".to_string();
    let r1: UpperBoundedRange<usize> = (..5).into();
    assert_eq!(&s[r1], "hello");

    let r2: UpperBoundedRange<usize> = (..=5).into();
    assert_eq!(&s[r2], "hello,");
}

#[test]
fn string_mut() {
    let mut s: String = "hello, world!".to_string();
    let r: UpperBoundedRange<usize> = (..5).into();

    unsafe {
        let b = s[r].as_bytes_mut();
        b[0] = b'y';
    }

    assert_eq!(s, "yello, world!");
}

#[test]
fn str() {
    let s: &str = "hello, world!";
    let r1: UpperBoundedRange<usize> = (..5).into();
    assert_eq!(&s[r1], "hello");

    let r2: UpperBoundedRange<usize> = (..=5).into();
    assert_eq!(&s[r2], "hello,");
}

#[test]
fn str_mut() {
    let mut outer: String = "hello, world!".to_string();
    let s: &mut str = outer.as_mut_str();
    let r: UpperBoundedRange<usize> = (..5).into();

    unsafe {
        let b = s[r].as_bytes_mut();
        b[0] = b'y';
    }

    assert_eq!(s, "yello, world!");
}

#[test]
fn slice() {
    let outer = vec![1, 2, 3, 4, 5];
    let slice: &[_] = &outer;
    let r: UpperBoundedRange<_> = (..=2).into();
    assert_eq!(slice[r], [1, 2, 3]);
}

#[test]
fn slice_mut() {
    let mut outer = vec![1, 2, 3, 4, 5];
    let slice: &mut [_] = &mut outer;
    let r: UpperBoundedRange<_> = (..2).into();
    *(slice[r].get_mut(0).unwrap()) = 7;
    assert_eq!(slice, [7, 2, 3, 4, 5]);
}

#[test]
fn vec() {
    let v: Vec<_> = vec![1, 2, 3, 4, 5];
    let r: UpperBoundedRange<_> = (..1).into();
    assert_eq!(v[r], vec![1]);
}

#[test]
fn vec_mut() {
    let mut v: Vec<_> = vec![1, 2, 3, 4, 5];
    let r: UpperBoundedRange<_> = (..=3).into();
    *(v[r].get_mut(1).unwrap()) = 7;
    assert_eq!(v, vec![1, 7, 3, 4, 5]);
}
