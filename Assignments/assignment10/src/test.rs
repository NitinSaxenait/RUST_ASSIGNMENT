#[cfg(test)]
mod tests {
    use crate::ques1_collection::hasmap_::sum_conditional;
    use crate::ques2_collection::add_duplicate::add_duplicate;
    use crate::ques2_collection::drop_from_vector::drop_from_list;
    use crate::ques2_collection::first_even_number::find_first_even;
    use crate::ques2_collection::is_palindrome::if_palindrome;
    use crate::ques2_collection::remove_continous_sequence::remove_continous_value;
    use crate::ques2_collection::reverse_list::make_reverse;
    use std::collections::HashMap;

    #[test]

    fn if_palindrome_success() {
        assert_eq!(
            if_palindrome(&vec![1, 5, 1]),
            Ok("Makes Palindrome".to_string())
        );
    }
    #[test]
    fn if_palindrome_failure() {
        assert_eq!(if_palindrome(&vec![1, 5, 4, 1]), Err("No".to_string()));
    }

    #[test]
    fn make_reverse_success() {
        assert_eq!(make_reverse(&vec![1, 2, 3]), Ok(vec![3, 2, 1]));
    }
    #[test]
    fn make_reverse_failure() {
        assert_eq!(make_reverse(&vec![]), Err("Empty list".to_string()));
    }

    #[test]
    fn find_first_even_success() {
        assert_eq!(find_first_even(&vec![4, 6, 8, 9]), Some(4));
    }
    #[test]
    fn find_first_even_failure() {
        assert_eq!(find_first_even(&vec![]), None);
    }

    #[test]
    fn remove_continous_success() {
        assert_eq!(
            remove_continous_value(&mut vec![1, 1, 1, 1, 2, 3, 3, 1, 1, 4, 5, 5, 5, 5]),
            Some(vec![1, 2, 3, 1, 4, 5])
        );
    }

    #[test]
    fn remove_continous_failure() {
        assert_eq!(remove_continous_value(&mut vec![]), None);
    }

    #[test]
    fn adding_duplicates() {
        assert_eq!(
            add_duplicate(&mut vec![1, 2, 3, 3, 4]),
            Ok(vec![1, 1, 2, 2, 3, 3, 3, 3, 4, 4])
        );
    }
    #[test]
    fn adding_duplicate_failure() {
        assert_eq!(add_duplicate(&mut vec![]), Err(0));
    }

    #[test]
    fn drop_from_list_success() {
        assert_eq!(
            drop_from_list(5, &mut vec![1, 2, 3, 4, 5]),
            Ok(&mut vec![1, 2, 3, 4])
        );
    }
    #[test]
    fn drop_from_list_failure() {
        assert_eq!(
            drop_from_list(3, &mut vec![]),
            Err("Sorry,your list is empty.".to_string())
        );
    }
    /// test cases defined for hashmap function.
    #[test]
    fn sum_conditional_success() {
        let mut take_value = HashMap::new();
        take_value.insert("chaman", 20);
        take_value.insert("naman", 19);
        take_value.insert("ankur", 30);
        take_value.insert("kusum", 25);
        take_value.insert("love", 18);
        assert_eq!(sum_conditional(take_value, "an"), Some(69));
    }
    #[test]
    fn sum_conditional_failure() {
        let mut take_value = HashMap::new();

        assert_eq!(sum_conditional(take_value, "shi"), None);
    }
    #[test]
    fn sum_conditional_true() {
        let mut take_value = HashMap::new();
        take_value.insert("lalu", 20);
        take_value.insert("kalu", 19);
        take_value.insert("molu", 30);
        take_value.insert("kusum", 25);
        take_value.insert("love", 18);
        assert_eq!(sum_conditional(take_value, "lu"), Some(69));
    }
}
