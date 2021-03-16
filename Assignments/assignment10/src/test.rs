#[cfg(test)]
mod tests {
    use crate::ques2_collection::add_duplicate::add_duplicate;
    use crate::ques2_collection::drop_from_vector::drop_from_list;
    use crate::ques2_collection::first_even_number::first_even;

    use crate::ques1_collection::hasmap_::sum_conditional;
    use crate::ques2_collection::is_palindrome::if_palindrome;
    use crate::ques2_collection::remove_continous_sequence::remove_continous_value;
    use crate::ques2_collection::reverse_list::make_reverse;
    use std::collections::HashMap;

    /// test cases for all the functions of collections questions

    #[test]
    /// for palindrome_true
    fn palindrome_true() {
        assert_eq!(if_palindrome(&vec![1, 5, 1]), "Makes Palindrome");
    }
    #[test]
    fn palindrome_is_true() {
        assert_eq!(if_palindrome(&vec![1, 5, 4, 1]), "Makes Palindrome");
    }
    /// checking for reverse is true
    #[test]
    fn reverse_true() {
        assert_eq!(make_reverse(&vec![1, 2, 3]), [3, 2, 1]);
    }
    #[test]
    fn reverse_is_true() {
        assert_eq!(make_reverse(&vec![1, 2, 3, 4, 5, 6]), [6, 5, 4, 3, 2, 1]);
    }
    /// test for checking if first value is even or not.
    #[test]
    fn first_even_true() {
        assert_eq!(first_even(&vec![4, 6, 8, 9]), 4);
    }
    #[test]
    fn first_even_is_true() {
        assert_eq!(first_even(&vec![10, 11, 21, 32]), 10);
    }
    /// test to check if value is removed or not successfully.
    #[test]
    fn remove_is_true() {
        assert_eq!(
            remove_continous_value(&mut vec![1, 1, 1, 1, 2, 3, 3, 1, 1, 4, 5, 5, 5, 5]),
            [1, 2, 3, 1, 4, 5]
        );
    }

    #[test]
    fn continous_values_removed() {
        assert_eq!(
            remove_continous_value(&mut vec![1, 1, 2, 3, 4, 4, 5]),
            [1, 2, 3, 4, 5]
        );
    }
    /// test for adding duplicate values in input vectors.
    #[test]
    fn adding_duplicates() {
        assert_eq!(
            add_duplicate(&mut vec![1, 2, 3, 3, 4]),
            [1, 1, 2, 2, 3, 3, 3, 3, 4, 4]
        );
    }
    #[test]
    fn adding_duplicate_values() {
        assert_eq!(
            add_duplicate(&mut vec![1, 2, 3, 3, 4]),
            [1, 1, 2, 2, 3, 3, 3, 3, 4, 4]
        );
    }
    /// test for check if value is removed from vector successfully or not.
    #[test]
    fn value_removed() {
        assert_eq!(
            drop_from_list(5, &mut vec![1, 2, 3, 4, 5]),
            &mut [1, 2, 3, 4]
        );
    }
    #[test]
    fn value_dropped() {
        assert_eq!(
            drop_from_list(3, &mut vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]),
            &mut [1, 2, 4, 5, 6, 7, 8, 9, 10, 11]
        );
    }
    /// test cases defined for hashmap function.
    #[test]
    fn sum_true() {
        let mut take_value = HashMap::new();
        take_value.insert("chaman", 20);
        take_value.insert("naman", 19);
        take_value.insert("ankur", 30);
        take_value.insert("kusum", 25);
        take_value.insert("love", 18);
        assert_eq!(sum_conditional(take_value, "an"), 69);
    }
    #[test]
    fn sum_age_true() {
        let mut take_value = HashMap::new();
        take_value.insert("devanshi", 20);
        take_value.insert("priyanshi", 19);
        take_value.insert("ankur", 30);
        take_value.insert("kusum", 25);
        take_value.insert("love", 18);
        assert_eq!(sum_conditional(take_value, "shi"), 39);
    }
    #[test]
    fn sum_is_true() {
        let mut take_value = HashMap::new();
        take_value.insert("lalu", 20);
        take_value.insert("kalu", 19);
        take_value.insert("molu", 30);
        take_value.insert("kusum", 25);
        take_value.insert("love", 18);
        assert_eq!(sum_conditional(take_value, "lu"), 69);
    }
}
