use crate::UnboundedRange;

#[test]
fn string() {
    let s: String = "hello, world!".to_string();
    let r = UnboundedRange;
    assert_eq!(&s[r], "hello, world!");
}

#[test]
fn string_mut() {
    let mut s: String = "hello, world!".to_string();
    let r = UnboundedRange;

    unsafe {
        let b = s[r].as_bytes_mut();
        b[4] = b'a';
    }

    assert_eq!(s, "hella, world!");
}

#[test]
fn str() {
    let s: &str = "hello, world!";
    let r = UnboundedRange;
    assert_eq!(&s[r], "hello, world!");
}

#[test]
fn str_mut() {
    let mut outer: String = "hello, world!".to_string();
    let s: &mut str = outer.as_mut_str();
    let r = UnboundedRange;

    unsafe {
        let b = s[r].as_bytes_mut();
        b[4] = b'a';
    }

    assert_eq!(s, "hella, world!");
}

#[test]
fn c_string() {
    let s = std::ffi::CString::new("hello, world!").unwrap();
    let r = UnboundedRange;
    assert_eq!(&s[r], s.as_c_str());
}

#[test]
fn os_string() {
    let s: std::ffi::OsString = "hello, world!".into();
    let r = UnboundedRange;
    assert_eq!(&s[r], s.as_os_str());
}

#[test]
fn os_string_mut() {
    let mut s: std::ffi::OsString = "hello, world!".into();
    let r = UnboundedRange;
    _ = &s[r].make_ascii_uppercase();
    assert_eq!(&s[r], std::ffi::OsString::from("HELLO, WORLD!"));
}

#[test]
fn slice() {
    let outer = vec![1, 2, 3, 4, 5];
    let slice: &[_] = &outer;
    let r = UnboundedRange;
    assert_eq!(slice[r], [1, 2, 3, 4, 5]);
}

#[test]
fn slice_mut() {
    let mut outer = vec![1, 2, 3, 4, 5];
    let slice: &mut [_] = &mut outer;
    let r = UnboundedRange;
    *(slice[r].get_mut(0).unwrap()) = 7;
    assert_eq!(slice, [7, 2, 3, 4, 5]);
}

#[test]
fn vec() {
    let v: Vec<_> = vec![1, 2, 3, 4, 5];
    let r = UnboundedRange;
    assert_eq!(v[r], vec![1, 2, 3, 4, 5]);
}

#[test]
fn vec_mut() {
    let mut v: Vec<_> = vec![1, 2, 3, 4, 5];
    let r = UnboundedRange;
    *(v[r].get_mut(1).unwrap()) = 7;
    assert_eq!(v, vec![1, 7, 3, 4, 5]);
}
