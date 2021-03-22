// test cases for repeated characters
//
// test cases
#[cfg(test)]
mod test {
    use crate::check_palindrome::check_palindrome;
    use crate::find_duplicate_charaters::count_same_characters;
    use crate::rotation_is_true::checking_rotation;
    //
    // test for duplicate string
    //
    #[test]
    fn duplicate_is_true() {
        assert_eq!(count_same_characters("hello world"), "lo")
    }
    #[test]
    fn duplicate_true() {
        assert_eq!(count_same_characters("apple"), "p")
    }
    #[test]
    fn duplicate_false() {
        assert_eq!(count_same_characters("samuel"), "")
    }
    //
    //test for rotation_true
    //
    #[test]
    fn rotation_false() {
        assert_eq!(checking_rotation("rahul", "rama"), false)
    }

    #[test]
    fn rotation_is_true() {
        assert_eq!(checking_rotation("nitin", "nitin"), true)
    }
    #[test]
    fn rotation__is_true() {
        assert_eq!(checking_rotation("mam", "mam"), true)
    }

    //
    // test cases for palindrome
    //
    #[test]
    fn true_palindrome() {
        assert!(check_palindrome("nitin", 0, 4));
    }
    #[test]
    fn true_for_palindrome() {
        assert!(check_palindrome("raar", 0, 3));
    }
    #[test]
    fn true_is_palindrome_() {
        assert!(check_palindrome("mam", 0, 2));
    }
}
