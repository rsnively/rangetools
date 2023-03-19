/// Integer types that have a *next* number.
///
/// Types are required to implement Step if a range of that type is to be
/// iterated through.
pub trait Step {
    /// Returns the next value, does not check for overflow.
    fn next(self) -> Self;
    /// Returns the previous value, does not check for overflow.
    fn prev(self) -> Self;

    /// Returns the number of successor steps required to get from `start` to `end`
    fn steps_between(start: &Self, end: &Self) -> Option<usize>;

    /// Returns the value that would be obtained by taking the successor of `start` `count` times.
    fn forward(start: Self, count: usize) -> Self;
    /// Returns the value that would be obtained by taking the predecessor of `start` `count` times.
    fn backward(start: Self, count: usize) -> Self;
}

impl Step for u8 {
    fn next(self) -> Self {
        self + 1
    }
    fn prev(self) -> Self {
        self - 1
    }
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if *start <= *end {
            Some((*end - *start) as usize)
        } else {
            None
        }
    }
    fn forward(start: Self, count: usize) -> Self {
        start + count as u8
    }
    fn backward(start: Self, count: usize) -> Self {
        start - count as u8
    }
}
impl Step for u16 {
    fn next(self) -> Self {
        self + 1
    }
    fn prev(self) -> Self {
        self - 1
    }
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if *start <= *end {
            Some((*end - *start) as usize)
        } else {
            None
        }
    }
    fn forward(start: Self, count: usize) -> Self {
        start + count as u16
    }
    fn backward(start: Self, count: usize) -> Self {
        start - count as u16
    }
}
impl Step for u32 {
    fn next(self) -> Self {
        self + 1
    }
    fn prev(self) -> Self {
        self - 1
    }
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if *start <= *end {
            Some((*end - *start) as usize)
        } else {
            None
        }
    }
    fn forward(start: Self, count: usize) -> Self {
        start + count as u32
    }
    fn backward(start: Self, count: usize) -> Self {
        start - count as u32
    }
}
impl Step for u64 {
    fn next(self) -> Self {
        self + 1
    }
    fn prev(self) -> Self {
        self - 1
    }
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if *start <= *end {
            Some((*end - *start) as usize)
        } else {
            None
        }
    }
    fn forward(start: Self, count: usize) -> Self {
        start + count as u64
    }
    fn backward(start: Self, count: usize) -> Self {
        start - count as u64
    }
}
impl Step for u128 {
    fn next(self) -> Self {
        self + 1
    }
    fn prev(self) -> Self {
        self - 1
    }
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if *start <= *end {
            Some((*end - *start) as usize)
        } else {
            None
        }
    }
    fn forward(start: Self, count: usize) -> Self {
        start + count as u128
    }
    fn backward(start: Self, count: usize) -> Self {
        start - count as u128
    }
}
impl Step for usize {
    fn next(self) -> Self {
        self + 1
    }
    fn prev(self) -> Self {
        self - 1
    }
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if *start <= *end {
            Some((*end - *start) as usize)
        } else {
            None
        }
    }
    fn forward(start: Self, count: usize) -> Self {
        start + count
    }
    fn backward(start: Self, count: usize) -> Self {
        start - count
    }
}

impl Step for i8 {
    fn next(self) -> Self {
        self + 1
    }
    fn prev(self) -> Self {
        self - 1
    }
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if *start <= *end {
            Some((*end - *start) as usize)
        } else {
            None
        }
    }
    fn forward(start: Self, count: usize) -> Self {
        start + count as i8
    }
    fn backward(start: Self, count: usize) -> Self {
        start - count as i8
    }
}
impl Step for i16 {
    fn next(self) -> Self {
        self + 1
    }
    fn prev(self) -> Self {
        self - 1
    }
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if *start <= *end {
            Some((*end - *start) as usize)
        } else {
            None
        }
    }
    fn forward(start: Self, count: usize) -> Self {
        start + count as i16
    }
    fn backward(start: Self, count: usize) -> Self {
        start - count as i16
    }
}
impl Step for i32 {
    fn next(self) -> Self {
        self + 1
    }
    fn prev(self) -> Self {
        self - 1
    }
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if *start <= *end {
            Some((*end - *start) as usize)
        } else {
            None
        }
    }
    fn forward(start: Self, count: usize) -> Self {
        start + count as i32
    }
    fn backward(start: Self, count: usize) -> Self {
        start - count as i32
    }
}
impl Step for i64 {
    fn next(self) -> Self {
        self + 1
    }
    fn prev(self) -> Self {
        self - 1
    }
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if *start <= *end {
            Some((*end - *start) as usize)
        } else {
            None
        }
    }
    fn forward(start: Self, count: usize) -> Self {
        start + count as i64
    }
    fn backward(start: Self, count: usize) -> Self {
        start - count as i64
    }
}
impl Step for i128 {
    fn next(self) -> Self {
        self + 1
    }
    fn prev(self) -> Self {
        self - 1
    }
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if *start <= *end {
            Some((*end - *start) as usize)
        } else {
            None
        }
    }
    fn forward(start: Self, count: usize) -> Self {
        start + count as i128
    }
    fn backward(start: Self, count: usize) -> Self {
        start - count as i128
    }
}
impl Step for isize {
    fn next(self) -> Self {
        self + 1
    }
    fn prev(self) -> Self {
        self - 1
    }
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if *start <= *end {
            Some((*end - *start) as usize)
        } else {
            None
        }
    }
    fn forward(start: Self, count: usize) -> Self {
        start + count as isize
    }
    fn backward(start: Self, count: usize) -> Self {
        start - count as isize
    }
}
