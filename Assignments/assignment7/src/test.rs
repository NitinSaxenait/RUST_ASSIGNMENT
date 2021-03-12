#[cfg(test)]

// mod test is used to perform all test cases for all functions.
//
// #Functions Tested.
//
// pattern matching, creating substring, comparing string

// 6 tests are performed
mod tests {
    use crate::creating_substring::generate_substring;
    use crate::pattern_matching::pattern_match;
    use crate::ques3::compare_string;
    //

    // 2 test for matching pattern
    #[test]
    fn pattern_match_check() {
        assert_eq!(
            pattern_match("pankaj chaudhary", "cha"),
            "pattern found at index 7".to_string()
        );
    }
    #[test]
    fn pattern_match_true() {
        assert_eq!(
            pattern_match("nitin", "t"),
            "pattern found at index 2".to_string()
        );
    }

    #[test]
    fn pattern_matched() {
        assert_eq!(
            pattern_match("iphone", "apple"),
            "pattern is not present in given string".to_string()
        )
    }

    // 3 test for generating substring for abc and 123
    #[test]
    fn substring_generated_true() {
        assert_eq!(
            generate_substring("abc"),
            ["abc", "a", "ab", "ac", "b", "bc", "c"]
        );
    }
    #[test]
    fn substring_is_again_true() {
        assert_eq!(
            generate_substring("KUP"),
            ["KUP", "K", "KU", "KP", "U", "UP", "P"]
        );
    }

    #[test]
    fn substring_is_true() {
        assert_eq!(
            generate_substring("123"),
            ["123", "1", "12", "13", "2", "23", "3"]
        );
    }

    // 3 test cases for comparing strings.
    #[test]
    fn check_test_true() {
        assert_eq!(compare_string("jjdhid", "ikjhjk", "rtysgi"), "itdsgk")
    }
    #[test]
    fn check_for_true() {
        assert_eq!(compare_string("cdef", "aejy", "fahf"), "aeey")
    }
    #[test]
    fn for_true() {
        assert_eq!(compare_string("pk", "mh", "az"), "az")
    }
}
