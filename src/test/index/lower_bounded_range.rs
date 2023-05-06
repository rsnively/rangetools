use crate::LowerBoundedRange;

#[test]
fn string() {
    let s: String = "hello, world!".to_string();
    let r: LowerBoundedRange<usize> = (7..).into();
    assert_eq!(&s[r], "world!");
}

#[test]
fn string_mut() {
    let mut s: String = "hello, world!".to_string();
    let r: LowerBoundedRange<usize> = (7..).into();

    unsafe {
        let b = s[r].as_bytes_mut();
        b[1] = b'u';
    }

    assert_eq!(s, "hello, wurld!");
}

#[test]
fn str() {
    let s: &str = "hello, world!";
    let r: LowerBoundedRange<usize> = (7..).into();
    assert_eq!(&s[r], "world!");
}

#[test]
fn str_mut() {
    let mut outer: String = "hello, world!".to_string();
    let s: &mut str = outer.as_mut_str();
    let r: LowerBoundedRange<usize> = (7..).into();

    unsafe {
        let b = s[r].as_bytes_mut();
        b[1] = b'u';
    }

    assert_eq!(s, "hello, wurld!");
}

#[test]
fn c_str() {
    let outer = std::ffi::CString::new("hello, world!").unwrap();
    let s = outer.as_c_str();
    let r: LowerBoundedRange<usize> = (7..).into();
    assert_eq!(&s[r], std::ffi::CString::new("world!").unwrap().as_c_str());
}

#[test]
fn slice() {
    let outer = vec![1, 2, 3, 4, 5];
    let slice: &[_] = &outer;
    let r: LowerBoundedRange<_> = (2..).into();
    assert_eq!(slice[r], [3, 4, 5]);
}

#[test]
fn slice_mut() {
    let mut outer = vec![1, 2, 3, 4, 5];
    let slice: &mut [_] = &mut outer;
    let r: LowerBoundedRange<_> = (1..).into();
    *(slice[r].get_mut(0).unwrap()) = 7;
    assert_eq!(slice, [1, 7, 3, 4, 5]);
}

#[test]
fn vec() {
    let v: Vec<_> = vec![1, 2, 3, 4, 5];
    let r: LowerBoundedRange<_> = (2..).into();
    assert_eq!(v[r], vec![3, 4, 5]);
}

#[test]
fn vec_mut() {
    let mut v: Vec<_> = vec![1, 2, 3, 4, 5];
    let r: LowerBoundedRange<_> = (2..).into();
    *(v[r].get_mut(0).unwrap()) = 7;
    assert_eq!(v, vec![1, 2, 7, 4, 5]);
}
