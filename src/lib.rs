/// This function divides two numbers.
///
/// # Example #1
///
/// ```
/// let result = doctests_exercise::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Example #2
///
/// ```
/// let result = doctests_exercise::div(6, 3);
/// assert_eq!(result, 2);
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// doctests_exercise::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }
    a / b
}

/// This function subtracts two numbers.
///
/// # Example #1
///
/// ```
/// let result = doctests_exercise::sub(9, 2);
/// assert_eq!(result, 7);
/// ```
///
/// # Example #2
///
/// ```
/// let result = doctests_exercise::sub(6, 9);
/// assert_eq!(result, -3);
/// ```
pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}
