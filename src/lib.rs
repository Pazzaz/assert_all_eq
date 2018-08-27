/// Asserts that multiple expressions are equal to each other (using [`PartialEq`]).
///
/// On panic, this macro will print the values of the differing expressions with their
/// debug representations.
///
/// Like `assert!` and `assert_eq!`, this macro has a second form, where a custom
/// panic message can be provided. To make parsing possible, `;` is used to seperate
/// the compared expressions from the panic message.
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

    // When only two expressions are compared, use `std::assert_eq!`
    ($first:expr , $second:expr) =>    ({ assert_eq!($first, $second) });
    ($first:expr , $second:expr ;) =>  ({ assert_eq!($first, $second) });
    ($first:expr , $second:expr ,) =>  ({ assert_eq!($first, $second) });
    ($first:expr , $second:expr ,;) => ({ assert_eq!($first, $second) });
    ($first:expr , $second:expr ; $($arg:tt)+) =>  ({ assert_eq!($first, $second, $($arg)+) });
    ($first:expr , $second:expr ,; $($arg:tt)+) => ({ assert_eq!($first, $second, $($arg)+) });

    ( $first:expr , $( $x:expr ),+ ;) => ({ assert_all_eq!( $first $( ,$x )+) });
    ( $first:expr , $( $x:expr ),+ ,;) => ({ assert_all_eq!( $first $( ,$x )+) });
    ( $first:expr , $( $x:expr ),+ ,) => ({ assert_all_eq!( $first $( ,$x )+) });
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
                    match &$x {
                        right_val => {
                            if !(*a == *right_val) {
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
                    match &$x {
                        right_val => {
                            if !(*a == *right_val) {
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
            &[1, 2, 3, 4],
        );
        assert_all_eq!(
            &[1, 2, 3, 4],
            &[1, 2, 3, 4],
            (1..=4).collect::<Vec<_>>().as_slice(),
            &[1, 2, 3, 4],
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

    #[test]
    fn minimum_comparisons() {
        use std::cell::RefCell;

        #[derive(Debug, Clone)]
        struct Test(u8, RefCell<usize>);
        impl PartialEq<Test> for Test {
            fn eq(&self, other: &Test) -> bool {
                let si = self.1.clone().into_inner();
                let oi = other.1.clone().into_inner();
                self.1.replace(si + 1);
                other.1.replace(oi + 1);

                self.0 == other.0
            }
        }
        let a = Test(1, RefCell::new(0));
        let b = Test(1, RefCell::new(0));
        let c = Test(1, RefCell::new(0));
        assert_all_eq!(a, b, c);
        let ai = a.1.into_inner();
        let bi = b.1.into_inner();
        let ci = c.1.into_inner();
        assert_eq!(ai + bi + ci, 4);

        let a = Test(1, RefCell::new(0));
        let b = Test(1, RefCell::new(0));
        let c = Test(1, RefCell::new(0));
        let d = Test(1, RefCell::new(0));
        let e = Test(1, RefCell::new(0));
        let f = Test(1, RefCell::new(0));
        assert_all_eq!(a, b, c, d, e, f);
        let ai = a.1.into_inner();
        let bi = b.1.into_inner();
        let ci = c.1.into_inner();
        let di = d.1.into_inner();
        let ei = e.1.into_inner();
        let fi = f.1.into_inner();
        assert_eq!(ai + bi + ci + di + ei + fi, 10);
    }

    #[test]
    fn two_true_format_zero() {
        assert_all_eq!(3, 3, 3; "Message");
        assert_all_eq!(3, 3; "Message");
    }

    #[test]
    fn two_true_format_one() {
        assert_all_eq!(3, 3, 3; "Message: {}", 1212);
        assert_all_eq!(3, 3; "Message: {}", 1212);
    }
    #[test]
    fn two_true_format_two() {
        assert_all_eq!(3, 3, 3; "Message: {}, {}", 1212, 5454);
        assert_all_eq!(3, 3; "Message: {}, {}", 1212, 5454);
    }
    #[test]
    fn trailing() {
        assert_all_eq!(3, 3, 3,);
        assert_all_eq!(3, 3, 3,;);
        assert_all_eq!(3, 3, 3;);
        assert_all_eq!(3, 3,;);
        assert_all_eq!(3, 3;);
    }
}
