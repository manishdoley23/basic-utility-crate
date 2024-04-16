/// This is a utility module for string operations
/// It contains functions to check if a string is palindrome, count occurrences of a character in a string and reverse a string
/// # Examples
/// ```
/// use lib_util_crate::string::check_if_palindrome;
/// use lib_util_crate::string::occurrences_of_char;
/// use lib_util_crate::string::reverse_string;
/// assert_eq!(check_if_palindrome("malayalam"), true);
/// assert_eq!(occurrences_of_char("aaaabbvcvzcxz", 'a'), 4);
/// assert_eq!(reverse_string("praman"), "namarp");
/// ```
pub fn check_if_palindrome(string: &str) -> bool {
    let mut start_idx: usize = 0;
    let mut last_idx: usize = string.len() - 1;

    while start_idx < last_idx {
        if string.chars().nth(start_idx) != string.chars().nth(last_idx) {
            return false;
        }
        start_idx += 1;
        last_idx -= 1;
    }

    true
}

pub fn occurrences_of_char(string: &str, check_chr: char) -> i32 {
    let mut count = 0;
    for chr in string.chars() {
        if chr == check_chr {
            count += 1
        }
    }
    count
}

pub fn reverse_string(string: &str) -> String {
    string.chars().rev().collect()
}

#[cfg(test)]
mod test_string {
    use crate::string::check_if_palindrome;
    use crate::string::occurrences_of_char;
    use crate::string::reverse_string;

    #[test]
    fn test_check_if_palindrome() {
        assert_eq!(check_if_palindrome("malayalam"), true);
    }

    #[test]
    fn test_check_occurrences_of_char() {
        assert_eq!(occurrences_of_char("aaaabbvcvzcxz", 'a'), 4);
    }

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("praman"), "namarp");
    }
}
