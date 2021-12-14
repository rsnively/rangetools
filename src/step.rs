/// Integer types that have a *next* number.
///
/// Types are required to implement Step if a range of that type is to be
/// iterated through.
pub trait Step {
    /// Returns the next value, does not check for overflow.
    fn next(self) -> Self;
}

impl Step for u8 {
    fn next(self) -> Self {
        self + 1
    }
}
impl Step for u16 {
    fn next(self) -> Self {
        self + 1
    }
}
impl Step for u32 {
    fn next(self) -> Self {
        self + 1
    }
}
impl Step for u64 {
    fn next(self) -> Self {
        self + 1
    }
}
impl Step for u128 {
    fn next(self) -> Self {
        self + 1
    }
}
impl Step for usize {
    fn next(self) -> Self {
        self + 1
    }
}

impl Step for i8 {
    fn next(self) -> Self {
        self + 1
    }
}
impl Step for i16 {
    fn next(self) -> Self {
        self + 1
    }
}
impl Step for i32 {
    fn next(self) -> Self {
        self + 1
    }
}
impl Step for i64 {
    fn next(self) -> Self {
        self + 1
    }
}
impl Step for i128 {
    fn next(self) -> Self {
        self + 1
    }
}
impl Step for isize {
    fn next(self) -> Self {
        self + 1
    }
}
