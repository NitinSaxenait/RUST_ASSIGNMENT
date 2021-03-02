#[cfg(test)]

mod tests {
    #[test]
    fn It_Works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn check_palindrome() {
        assert_eq!(check_palindrome("abba"), "Palindrome");
    }

    #[test]
    fn find_rep_char() {
        assert_eq!(find_rep_char("Hello World"), "lo");
    }

    #[test]
    fn check_input_string() {
        assert_eq!(check_str_rev("abcd","dcba"), "Matched");
    }
}