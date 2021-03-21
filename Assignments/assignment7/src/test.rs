#[cfg(test)]

mod test {
    use crate::ques1_assignment7::generating_substring::generate_substring;
    use crate::ques1_assignment7::pattern_matching::match_pattern;
    use crate::ques2_assignment7::get_desired_output::comparing_string;
    #[test]
    fn substring_generation_success() {
        assert_eq!(
            generate_substring("abc"),
            ["abc", "a", "ab", "ac", "b", "bc", "c"]
        );
    }

    #[test]
    fn substring_successfully_generated() {
        assert_eq!(
            generate_substring("KUP"),
            ["KUP", "K", "KU", "KP", "U", "UP", "P"]
        );
    }

    #[test]
    fn substring_generation_is_success() {
        assert_eq!(
            generate_substring("123"),
            ["123", "1", "12", "13", "2", "23", "3"]
        );
    }

    #[test]
    fn pattern_matching_success() {
        assert_eq!(
            match_pattern("pankaj chaudhary", "cha"),
            Ok("pattern found at index 7".to_string())
        );
    }
    #[test]
    fn pattern_match_success() {
        assert_eq!(
            match_pattern("nitin", "t"),
            Ok("pattern found at index 2".to_string())
        );
    }

    #[test]
    fn pattern_matching_failure() {
        assert_eq!(
            match_pattern("iphone", "apple"),
            Err("pattern is not present in given string".to_string())
        )
    }

    #[test]
    fn get_desired_output_success() {
        assert_eq!(
            comparing_string("jjdhid", "ikjhjk", "rtysgi"),
            Some("itdsgk".to_string())
        )
    }
    #[test]
    fn desired_output_success() {
        assert_eq!(
            comparing_string("cdef", "aejy", "fahf"),
            Some("aeey".to_string())
        )
    }
    #[test]
    fn get_desired_output_failure() {
        assert_eq!(comparing_string("", "", ""), None)
    }
}
