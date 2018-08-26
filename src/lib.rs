#![feature(refcell_replace_swap)]
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
        use std::fmt::Debug;
        match &$first {
            a => {
                let mut b = 0usize;

                // Seperate function to reduce compile time of macro
                fn not_eq<A, B>(left: A, right: B, i: usize)
                where A: Debug,
                      B: Debug,
                {
                    let index = format!("{}", i);
                    let pad = " ".repeat(index.len());
                    panic!(r#"equality assertion failed at position 0 and {i}
{pad}0: `{:?}`,
 {i}: `{:?}`"#, left, right, pad=pad, i=index);
                }
                $(
                    b += 1usize;
                    match (a, &$x) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                not_eq(left_val, right_val, b);
                            }
                        }
                    }
                )*
            }
        }
    });

    ( $first:expr , $( $x:expr ),+; $($arg:tt)+) => ({
        use std::fmt::Debug;
        match &$first {
            a => {
                let f = || format!($($arg)+);
                let mut b = 0usize;
                fn not_eq<A, B>(left: A, right: B, i: usize, f: &str)
                where A: Debug,
                      B: Debug,
                {
                    let index = format!("{}", i);
                    let pad = " ".repeat(index.len());
                    panic!(r#"equality assertion failed at position 0 and {i}
{pad}0: `{:?}`,
 {i}: `{:?}`: {message}"#, left, right, pad=pad, i=index, message=f);
                }
                $(
                    b += 1usize;
                    match (a, &$x) {
                        (left_val, right_val) => {
                            if !(*left_val == *right_val) {
                                not_eq(left_val, right_val, b, &f());
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
        fn a() -> &'static str {
            "yes"
        }
        fn b() -> &'static str {
            "yes"
        }
        fn c() -> &'static str {
            "yes"
        }
        assert_all_eq!(a(), b(), c());
    }
    #[test]
    #[should_panic]
    fn functions_false() {
        fn a() -> &'static str {
            "yes"
        }
        fn b() -> &'static str {
            "yes"
        }
        assert_all_eq!(a(), b(), "no");
    }
    #[test]
    fn mixed_true() {
        let a = || 1 + 2 + 3;
        fn b() -> i32 {
            6
        }
        let c = 6;
        let d = Box::new(6);
        assert_all_eq!(a(), b(), c, *d, 6, { 3 * 2 });
    }

    #[test]
    fn lifetimes_true() {
        assert_all_eq!((1..=4).collect::<Vec<_>>().as_slice(), &[1, 2, 3, 4]);
        assert_all_eq!(
            (1..=4).collect::<Vec<_>>().as_slice(),
            &[1, 2, 3, 4],
            &[1, 2, 3, 4]
        );
    }

    #[test]
    fn long_true() {
        assert_all_eq!(
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
        );
    }
    #[test]
    #[should_panic]
    fn long_false() {
        assert_all_eq!(
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
        );
    }

    use std::cell::RefCell;

    #[test]
    fn minimum_comparisons() {
        #[derive(Debug, Clone)]
        struct Test<T> {
            inner: T,
            compared: RefCell<usize>,
        }
        impl<T> Test<T> {
            fn new(i: T) -> Test<T> {
                Test {
                    inner: i,
                    compared: RefCell::new(0),
                }
            }
        }
        impl<T> PartialEq<Test<T>> for Test<T>
        where
            T: PartialEq,
        {
            fn eq(&self, other: &Test<T>) -> bool {
                self.compared.replace_with(|&mut old| old + 1);
                other.compared.replace_with(|&mut old| old + 1);
                self.inner == other.inner
            }
        }
        let a = Test::new(1);
        let b = Test::new(1);
        let c = Test::new(1);
        assert_all_eq!(a, b, c);
        let ai = a.compared.into_inner();
        let bi = b.compared.into_inner();
        let ci = c.compared.into_inner();
        assert_eq!(ai + bi + ci, 4);

        let a = Test::new(1);
        let b = Test::new(1);
        let c = Test::new(1);
        let d = Test::new(1);
        let e = Test::new(1);
        let f = Test::new(1);
        assert_all_eq!(a, b, c, d, e, f);
        let ai = a.compared.into_inner();
        let bi = b.compared.into_inner();
        let ci = c.compared.into_inner();
        let di = d.compared.into_inner();
        let ei = e.compared.into_inner();
        let fi = f.compared.into_inner();
        assert_eq!(ai + bi + ci + di + ei + fi, 10);
    }

    #[test]
    fn two_true_format_zero() {
        assert_all_eq!(3, 3; "Message");
    }

    #[test]
    fn two_true_format_one() {
        assert_all_eq!(3, 3; "Message: {}", 1212);
    }
    #[test]
    fn two_true_format_two() {
        assert_all_eq!(3, 3; "Message: {}, {}", 1212, 5454);
    }
    #[test]
    fn trailing() {
        assert_all_eq!(3,3,);
        assert_all_eq!(3,3,;);
        assert_all_eq!(3,3;);
    }
}
