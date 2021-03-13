//
// this is testing the matching_result function
//
// 3 test cases for checking:
// input_number is Even or Not.
#[cfg(test)]
mod tests {
    use crate::error_handling_on_evennumber::matching_result;

    #[test]
    fn found_even() {
        let output = matching_result(2);
        assert_eq!(output, "True => EVEN Number");
    }
    #[test]
    fn not_found_even() {
        let output = matching_result(23);
        assert_eq!(output, "NOT EVEN Number");
    }
    #[test]
    fn found_even_again() {
        let output = matching_result(98);
        assert_eq!(output, "True => EVEN Number");
    }
}
