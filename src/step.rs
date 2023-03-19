/// Types are required to implement this trait for ranges of that type to be
/// iterated through.
///
/// Just `std::iter::Step` without the nightly features.
///
/// Objects that have a notion of succesor and predecessor operations.
///
/// The successor operation moves towards values that compare greater.
/// The predecessor operation moves towards values that compare lesser.
pub trait Step: Clone + PartialOrd + Sized {
    /// Returns the number of succesor steps required to get from `start` to `end`.
    ///
    /// Returns `None` if the number of steps would overflow `usize`, or is infinite
    /// or if `start` > `end`.
    fn steps_between(start: &Self, end: &Self) -> Option<usize>;

    /// Returns the value that would be obtained by taking the successor of `start`
    /// `count` times.
    ///
    /// Returns `None` if this would overflow the range of values supported by `Self`.
    fn forward_checked(start: Self, count: usize) -> Option<Self>;

    /// Returns the value that would be obtained by taking the successor of `start`
    /// `count` times.
    ///
    /// If this would overflow the range of values supported by `Self`, this function is
    /// allowed to panic, wrap, or saturate.
    /// The suggested behavior is to panic when debug assertions are enabled, and to wrap
    /// or saturate otherwise.
    fn forward(start: Self, count: usize) -> Self {
        Step::forward_checked(start, count).expect("Overflow in `Step::forward`")
    }

    /// Returns the value that would be obtained by taking the predecessor of `start`
    /// `count` times.
    ///
    /// Returns `None` if this would overflow the range of values supported by `Self`.
    fn backward_checked(start: Self, count: usize) -> Option<Self>;

    /// Returns the value that would be obtained by taking the predecessor of `start`
    /// `count` times.
    ///
    /// If this would overflow the range of values supported by `Self`, this function is
    /// allowed to panic, wrap, or saturate.
    /// The suggested behavior is to panic when debug assertions are enabled, and to wrap
    /// or saturate otherwise.
    fn backward(start: Self, count: usize) -> Self {
        Step::backward_checked(start, count).expect("Overflow in `Step::backward`")
    }
}

macro_rules! step_identical_methods {
    () => {
        #[inline]
        #[allow(arithmetic_overflow)]
        fn forward(start: Self, n: usize) -> Self {
            // In debug builds, trigger a panic on overflow.
            // This should optimize completely out in release builds.
            if Self::forward_checked(start, n).is_none() {
                let _ = Self::MAX + 1;
            }
            // Do wrapping math to allow e.g. `Step::forward(-128i8, 255)`.
            start.wrapping_add(n as Self)
        }

        #[inline]
        #[allow(arithmetic_overflow)]
        fn backward(start: Self, n: usize) -> Self {
            // In debug builds, trigger a panic on overflow.
            // This should optimize completely out in release builds.
            if Self::backward_checked(start, n).is_none() {
                let _ = Self::MIN - 1;
            }
            // Do wrapping math to allow e.g. `Step::backward(127i8, 255)`.
            start.wrapping_sub(n as Self)
        }
    };
}

macro_rules! step_integer_impls {
    {
        narrower than or same width as usize:
            $( [ $u_narrower:ident $i_narrower:ident ] ),+;
        wider than usize:
            $( [ $u_wider:ident $i_wider:ident ] ),+;
    } => {
        $(
            #[allow(unreachable_patterns)]
            impl Step for $u_narrower {
                step_identical_methods!();

                #[inline]
                fn steps_between(start: &Self, end: &Self) -> Option<usize> {
                    if *start <= *end {
                        // This relies on $u_narrower <= usize
                        Some((*end - *start) as usize)
                    } else {
                        None
                    }
                }

                #[inline]
                fn forward_checked(start: Self, n: usize) -> Option<Self> {
                    match Self::try_from(n) {
                        Ok(n) => start.checked_add(n),
                        Err(_) => None, // if n is out of range, `unsigned_start + n` is too
                    }
                }

                #[inline]
                fn backward_checked(start: Self, n: usize) -> Option<Self> {
                    match Self::try_from(n) {
                        Ok(n) => start.checked_sub(n),
                        Err(_) => None, // if n is out of range, `unsigned_start - n` is too
                    }
                }
            }

            #[allow(unreachable_patterns)]
            impl Step for $i_narrower {
                step_identical_methods!();

                #[inline]
                fn steps_between(start: &Self, end: &Self) -> Option<usize> {
                    if *start <= *end {
                        // This relies on $i_narrower <= usize
                        //
                        // Casting to isize extends the width but preserves the sign.
                        // Use wrapping_sub in isize space and cast to usize to compute
                        // the difference that might not fit inside the range of isize.
                        Some((*end as isize).wrapping_sub(*start as isize) as usize)
                    } else {
                        None
                    }
                }

                #[inline]
                fn forward_checked(start: Self, n: usize) -> Option<Self> {
                    match $u_narrower::try_from(n) {
                        Ok(n) => {
                            // Wrapping handles cases like
                            // `Step::forward(-120_i8, 200) == Some(80_i8)`,
                            // even though 200 is out of range for i8.
                            let wrapped = start.wrapping_add(n as Self);
                            if wrapped >= start {
                                Some(wrapped)
                            } else {
                                None // Addition overflowed
                            }
                        }
                        // If n is out of range of e.g. u8,
                        // then it is bigger than the entire range for i8 is wide
                        // so `any_i8 + n` necessarily overflows i8.
                        Err(_) => None,
                    }
                }

                #[inline]
                fn backward_checked(start: Self, n: usize) -> Option<Self> {
                    match $u_narrower::try_from(n) {
                        Ok(n) => {
                            // Wrapping handles cases like
                            // `Step::forward(-120_i8, 200) == Some(80_i8)`,
                            // even though 200 is out of range for i8.
                            let wrapped = start.wrapping_sub(n as Self);
                            if wrapped <= start {
                                Some(wrapped)
                            } else {
                                None // Subtraction overflowed
                            }
                        }
                        // If n is out of range of e.g. u8,
                        // then it is bigger than the entire range for i8 is wide
                        // so `any_i8 - n` necessarily overflows i8.
                        Err(_) => None,
                    }
                }
            }
        )+

        $(
            #[allow(unreachable_patterns)]
            impl Step for $u_wider {
                step_identical_methods!();

                #[inline]
                fn steps_between(start: &Self, end: &Self) -> Option<usize> {
                    if *start <= *end {
                        usize::try_from(*end - *start).ok()
                    } else {
                        None
                    }
                }

                #[inline]
                fn forward_checked(start: Self, n: usize) -> Option<Self> {
                    start.checked_add(n as Self)
                }

                #[inline]
                fn backward_checked(start: Self, n: usize) -> Option<Self> {
                    start.checked_sub(n as Self)
                }
            }

            #[allow(unreachable_patterns)]
            impl Step for $i_wider {
                step_identical_methods!();

                #[inline]
                fn steps_between(start: &Self, end: &Self) -> Option<usize> {
                    if *start <= *end {
                        match end.checked_sub(*start) {
                            Some(result) => usize::try_from(result).ok(),
                            // If the difference is too big for e.g. i128,
                            // it's also gonna be too big for usize with fewer bits.
                            None => None,
                        }
                    } else {
                        None
                    }
                }

                #[inline]
                fn forward_checked(start: Self, n: usize) -> Option<Self> {
                    start.checked_add(n as Self)
                }

                #[inline]
                fn backward_checked(start: Self, n: usize) -> Option<Self> {
                    start.checked_sub(n as Self)
                }
            }
        )+
    };
}

#[cfg(target_pointer_width = "64")]
step_integer_impls! {
    narrower than or same width as usize: [u8 i8], [u16 i16], [u32 i32], [u64 i64], [usize isize];
    wider than usize: [u128 i128];
}

#[cfg(target_pointer_width = "32")]
step_integer_impls! {
    narrower than or same width as usize: [u8 i8], [u16 i16], [u32 i32], [usize isize];
    wider than usize: [u64 i64], [u128 i128];
}

#[cfg(target_pointer_width = "16")]
step_integer_impls! {
    narrower than or same width as usize: [u8 i8], [u16 i16], [usize isize];
    wider than usize: [u32 i32], [u64 i64], [u128 i128];
}

impl Step for char {
    #[inline]
    fn steps_between(&start: &char, &end: &char) -> Option<usize> {
        let start = start as u32;
        let end = end as u32;
        if start <= end {
            let count = end - start;
            if start < 0xD800 && 0xE000 <= end {
                usize::try_from(count - 0x800).ok()
            } else {
                usize::try_from(count).ok()
            }
        } else {
            None
        }
    }

    #[inline]
    fn forward_checked(start: char, count: usize) -> Option<char> {
        let start = start as u32;
        let mut res = Step::forward_checked(start, count)?;
        if start < 0xD800 && 0xD800 <= res {
            res = Step::forward_checked(res, 0x800)?;
        }
        if res <= char::MAX as u32 {
            // SAFETY: res is a valid unicode scalar
            // (below 0x110000 and not in 0xD800..0xE000)
            Some(unsafe { char::from_u32_unchecked(res) })
        } else {
            None
        }
    }

    #[inline]
    fn backward_checked(start: char, count: usize) -> Option<char> {
        let start = start as u32;
        let mut res = Step::backward_checked(start, count)?;
        if start >= 0xE000 && 0xE000 > res {
            res = Step::backward_checked(res, 0x800)?;
        }
        // SAFETY: res is a valid unicode scalar
        // (below 0x110000 and not in 0xD800..0xE000)
        Some(unsafe { char::from_u32_unchecked(res) })
    }
}
