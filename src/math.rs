/// This module is designed to return the factorial of a number, gcd of two numbers and check if a number is prime or not.
/// The factorial function takes a number as an argument and returns the factorial of that number.
/// The gcd function takes two numbers as arguments and returns the greatest common divisor of those numbers.
/// The check_for_prime function takes a number as an argument and returns a boolean value indicating if the number is prime or not.
///
/// # Examples:
/// ```
/// use lib_util_crate::math::{factorial, gcd, check_for_prime};
/// let factorial = factorial(5);
/// let gcd = gcd(100, 50);
/// let is_prime = check_for_prime(79);
/// ```
///
/// # Panics:
/// The factorial function will panic if the number is negative or a floating point number.
/// The gcd function will panic if the numbers are negative or floating point numbers.
/// The check_for_prime function will panic if the number is negative or a floating point number.
pub fn factorial(num: i32) -> i32 {
    let mut factorial = 1;
    for val in (1..=num).rev() {
        factorial = factorial * val;
        println!("{}", factorial);
    }
    factorial
}

pub fn gcd(num_one: i32, num_two: i32) -> i32 {
    let mut min_num = num_one.min(num_two);
    let mut max_num = num_one.max(num_two);

    let mut rem = 1;
    while rem != 0 {
        rem = max_num % min_num;
        max_num = min_num;
        min_num = rem;
    }

    max_num
}

pub fn check_for_prime(num: i32) -> bool {
    let mid_index = num / 2;
    for index in 2..=mid_index {
        if num % index == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::{check_for_prime, factorial, gcd};

    #[test]
    fn test_factorial() {
        let factorial = factorial(5);
        assert_eq!(factorial, 120);
    }

    #[test]
    fn test_gcd() {
        let gcd = gcd(100, 50);
        assert_eq!(gcd, 50);
    }

    #[test]
    fn test_check_for_prime() {
        let is_prime = check_for_prime(79);
        assert_eq!(is_prime, true);
    }
}
