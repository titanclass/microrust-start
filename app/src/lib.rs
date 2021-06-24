#![cfg_attr(not(test), no_std)]

/// A demonstration of writing some application library code that is able
/// to run in any no_std environment, and can be tested nicely.
/// ```
/// let result: Result<bool, &str> = Ok(true);
/// assert!(app::is_result_ok_and_true(result));
/// ```
pub fn is_result_ok_and_true<T>(result: Result<bool, T>) -> bool {
    matches!(result, Ok(true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_being_false() {
        let result: Result<bool, &str> = Err("whoops!");
        assert_eq!(is_result_ok_and_true(result), false);
    }
}
