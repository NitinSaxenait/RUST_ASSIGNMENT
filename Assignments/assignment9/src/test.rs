#[cfg(test)]
// this is used to test all cases for functions:
//
// minimum_value between two integer using generic.
// sort any type array using generic.

mod tests {
    use crate::custom_iterator_using_generic::{Iterator, GeometricSeries};
    use crate::min_of_two_using_generic::minimum_maximum;
    use crate::sorting_array_using_generic::generic_sort;
    //
    // test cases for minimum value.
    //
    #[test]
    fn minimum_came_true() {
        assert_eq!(minimum_maximum(9, 55), "The Minimum of two no is -> 9");
    }
    #[test]
    fn is_minimum() {
        assert_eq!(minimum_maximum(99, 999), "The Minimum of two no is -> 99");
    }
    #[test]
    fn minimum_test() {
        assert_eq!(minimum_maximum(124, 1), "The Minimum of two no is -> 1");
    }
    //
    // test cases for sorting.
    //
    #[test]
    fn sort_integer() {
        assert_eq!(generic_sort(&mut [9, 8, 7, 6, 5]), [5, 6, 7, 8, 9]);
    }

    #[test]
    fn sort_characters() {
        assert_eq!(generic_sort(&mut ["b", "a"]), ["a", "b"]);
    }
    #[test]
    fn sort_double() {
        assert_eq!(
            generic_sort(&mut [9.0, 8.5, 6.7, 5.2]),
            [5.2, 6.7, 8.5, 9.0]
        )
    }
    //
    // test cases for geometric_series function
    //
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
    fn series_came_true() {
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
    fn series_is_true() {
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
