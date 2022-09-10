/// Simple calculation
///
/// Adding unit testing
///
/// # Examples
/// ```
/// let result = doccomments::simple_calc(10, 2);
/// assert_eq!(result, 20);
/// ```
pub fn simple_calc(first_arg: u32, second_arg: u32) -> u32 {
    if first_arg < second_arg {
        println!("The first argument needs to be higher than the second!!");
        return 0;
    }

    first_arg * second_arg
}

#[cfg(test)]
mod simple_calc_tests {
    use super::*;

    #[test]
    fn simple_calc_works () {
        assert_eq!(simple_calc(5, 3), 15);
        assert_eq!(simple_calc(6, 1), 6);
        assert_eq!(simple_calc(1, 6), 0);
    }

    #[test]
    #[should_panic]
    fn simple_calc_failure () {
        assert_eq!(simple_calc(1, 6), 5);
    }
}

// This function checks if a number is even
pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(4));
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(5));
    }
}