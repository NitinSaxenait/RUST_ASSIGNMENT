#[cfg(test)]
mod test {
    use crate::ques1_assignment9::get_minimum_2_using_generics::get_values;
    use crate::ques1_assignment9::sorting_array_using_generics::generic_sorting;
    use crate::ques2_assignment9::custom_iterator_using_generics::GeometricSeries;
    use crate::ques2_assignment9::custom_iterator_using_generics::Iterator;
    #[test]
    fn sorting_integer_success() {
        assert_eq!(generic_sorting(&mut [9, 8, 7, 6, 5]), [5, 6, 7, 8, 9]);
    }

    #[test]
    fn sorting_characters_success() {
        assert_eq!(generic_sorting(&mut ["b", "a"]), ["a", "b"]);
    }

    #[test]
    fn sorting_double_success() {
        assert_eq!(
            generic_sorting(&mut [9.0, 8.5, 6.7, 5.2]),
            [5.2, 6.7, 8.5, 9.0]
        )
    }
    #[test]
    fn minimum_came_true() {
        assert_eq!(get_values(9, 55), "The Minimum of two no is -> 9");
    }
    #[test]
    fn is_minimum() {
        assert_eq!(get_values(99, 999), "The Minimum of two no is -> 99");
    }
    #[test]
    fn minimum_test_success() {
        assert_eq!(get_values(124, 1), "The Minimum of two no is -> 1");
    }
    #[test]
    fn geometric_series_true() {
        let take = GeometricSeries {
            first_number: 1,
            current_number: 6,
            ratio: 6,
        };
        assert_eq!(
            take.geometric_series(),
            [1, 6, 36, 216, 1296, 7776, 46656, 279936, 1679616, 10077696, 60466176]
        );
    }
    #[test]
    fn series_give_success() {
        let take = GeometricSeries {
            first_number: 1,
            current_number: 2,
            ratio: 2,
        };
        assert_eq!(
            take.geometric_series(),
            [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]
        );
    }
    #[test]
    fn series_came_true() {
        let take = GeometricSeries {
            first_number: 2,
            current_number: 4,
            ratio: 2,
        };
        assert_eq!(
            take.geometric_series(),
            [2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048]
        );
    }
}
