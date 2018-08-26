/// Asserts that multiple expressions are equal to each other (using [`PartialEq`]).
///
/// On panic, this macro will print the values of the differing expressions with their
/// debug representations.
///
/// Like `assert_eq!` and `assert!`, this macro has a second form, where a custom
/// panic message can be provided.
///
/// # Examples
///
/// ```
/// let a = 3;
/// let b = 2 + 1;
/// let c = 1 + 1 + 1;
/// assert_all_eq!(a, b, c);
/// assert_all_eq!(a, b, c, 3, 3, 3, 3, 3, 3);
///
/// assert_all_eq!(a, b, c; "we are testing addition with {}, {} and {}", a, b, c);
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
/// let a = 3;
/// let b = 2 + 1;
/// let c = 1 + 1 + 1;
/// debug_assert_all_eq!(a, b, c);
/// ```
#[macro_export]
macro_rules! debug_assert_all_eq {
    ($($arg:tt)*) => (if cfg!(debug_assertions) { assert_all_eq!($($arg)*); })
}
