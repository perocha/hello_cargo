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

#[test]
fn simple_calc_test () {
    assert_eq!(simple_calc(5, 3), 15);
    assert_eq!(simple_calc(6, 1), 6);
    assert_eq!(simple_calc(1, 6), 0);
}

#[test]
#[should_panic]
fn simple_calc_test_failure () {
    assert_eq!(simple_calc(1, 6), 5);
}