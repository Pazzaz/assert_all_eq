/// Asserts that multiple expressions are equal to each other (using [`PartialEq`]).
///
/// On panic, this macro will print the values of the differing expressions with their
/// debug representations.
///
/// Like `assert!` and `assert_eq!`, this macro has a second form, where a custom
/// panic message can be provided.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate assert_all_eq;
///
/// fn main() {
///     let a = 3;
///     let b = 2 + 1;
///     let c = 1 + 1 + 1;
///     assert_all_eq!(a, b, c);
///     assert_all_eq!(a, b, c, 3, 3, 3, 3, 3, 3);
///
///     assert_all_eq!(a, b, c; "we are testing addition with {}, {} and {}", a, b, c);
/// }
/// ```
#[macro_export]
macro_rules! assert_all_eq {
    ($left:expr , $right:expr) =>    ({ assert_eq!($left, $right) });
    ($left:expr , $right:expr ;) =>  ({ assert_eq!($left, $right) });
    ($left:expr , $right:expr ,) =>  ({ assert_eq!($left, $right) });
    ($left:expr , $right:expr ,;) => ({ assert_eq!($left, $right) });
    ($left:expr , $right:expr ; $($arg:tt)+) =>  ({ assert_eq!($left, $right, $($arg)+) });
    ($left:expr , $right:expr ,; $($arg:tt)+) => ({ assert_eq!($left, $right, $($arg)+) });

    ( $first:expr , $( $x:expr ),+ ;) => ({ assert_all_eq!( $first $( ,$x )+) });
    ( $first:expr , $( $x:expr ),+ ,; $($arg:tt)+) => ({ assert_all_eq!($first $( ,$x )+; $($arg)+) });
    ( $first:expr , $( $x:expr ),+) => ({
        match &$first {
            a => {
                let mut b = 0usize;
                let not_eq = |left, right, i| {
                    let index = format!("{}", i);
                    let pad = " ".repeat(index.len());
                    panic!(r#"equality assertion failed at position 0 and {i}
{pad}0: `{:?}`,
 {i}: `{:?}`"#, left, right, pad=pad, i=index);
                };
                $(
                    match (a, &$x) {
                        (left_val, right_val) => {
                            b += 1usize;
                            if !(*left_val == *right_val) {
                                not_eq(*left_val, *right_val, b);
                            }
                        }
                    }
                )*

            }
        }
    });

    ( $first:expr , $( $x:expr ),+; $($arg:tt)+) => ({
        match &$first {
            a => {
                let mut b = 0usize;
                let not_eq = |left, right, i| {
                    let index = format!("{}", i);
                    let pad = " ".repeat(index.len());
                    panic!(r#"equality assertion failed at position 0 and {i}
{pad}0: `{:?}`,
 {i}: `{:?}`: {}"#, left, right, format_args!($($arg)+), pad=pad, i=index);
                };
                $(
                    match (a, &$x) {
                        (left_val, right_val) => {
                            b += 1usize;
                            if !(*left_val == *right_val) {
                                not_eq(*left_val, *right_val, b);
                            }
                        }
                    }
                )*
            }
        }
    });
}

/// Asserts that multiple expressions are equal to each other (using [`PartialEq`]).
///
/// On panic, this macro will print the values of the differing expressions with their
/// debug representations.
///
/// Unlike [`assert_all_eq!`], `debug_assert_all_eq!` statements are only enabled in non
/// optimized builds by default. An optimized build will omit all
/// `debug_assert_all_eq!` statements unless `-C debug-assertions` is passed to the
/// compiler. This makes `debug_assert_all_eq!` useful for checks that are too
/// expensive to be present in a release build but may be helpful during
/// development.
///
/// [`assert_all_eq!`]: macro.assert_all_eq.html
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate assert_all_eq;
///
/// fn main() {
///     let a = 3;
///     let b = 2 + 1;
///     let c = 1 + 1 + 1;
///     debug_assert_all_eq!(a, b, c);
/// }
/// ```
#[macro_export]
macro_rules! debug_assert_all_eq {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { assert_all_eq!($($arg)*); })
}

#[cfg(test)]
mod tests {
    #[test]
    fn two_true() {
        assert_all_eq!(3, 3);
    }
    #[test]
    #[should_panic]
    fn two_false() {
        assert_all_eq!(4, 3);
    }
    #[test]
    fn three_true() {
        assert_all_eq!(3, 3, 3);
    }
    #[test]
    #[should_panic]
    fn three_false() {
        assert_all_eq!(3, 3, 4);
    }
    #[test]
    fn functions_true() {
        fn a() -> &'static str {"yes"}
        fn b() -> &'static str {"yes"}
        fn c() -> &'static str {"yes"}
        assert_all_eq!(a(), b(), c());
    }
    #[test]
    #[should_panic]
    fn functions_false() {
        fn a() -> &'static str {"yes"}
        fn b() -> &'static str {"yes"}
        assert_all_eq!(a(), b(), "no");
    }
    #[test]
    fn mixed_true() {
        let a = || {1+2+3};
        fn b() -> i32 {6}
        let c = 6;
        let d = Box::new(6);
        assert_all_eq!(a(), b(), c, *d, 6, {3*2});
    }

    #[test]
    fn lifetimes_true() {
        assert_all_eq!((1..=4).collect::<Vec<_>>().as_slice(), &[1,2,3,4]);
        assert_all_eq!((1..=4).collect::<Vec<_>>().as_slice(), &[1,2,3,4], &[1,2,3,4]);
    }

    #[test]
    fn long_true() {
        assert_all_eq!(1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1);
    }
    #[test]
    #[should_panic]
    fn long_false() {
        assert_all_eq!(1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1);
    }

}