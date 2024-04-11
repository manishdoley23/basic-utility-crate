pub fn check_if_palindrome(string: &str) -> bool {
    let mut start_idx: usize = 0;
    let mut last_idx: usize = string.len();

    while start_idx < last_idx {
        if string.get(start_idx) != string.get(last_idx) {
            return false;
        }
    }

    true
}
