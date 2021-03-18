#[cfg(test)]
mod tests {
    use crate::error_handling::error_handling_on_even_number::matching_result;

    #[test]
    fn found_even_success() {
        let output = matching_result(2);
        assert_eq!(output, "True => EVEN Number");
    }
    #[test]
    fn found_even_failure() {
        let output = matching_result(23);
        assert_eq!(output, "NOT EVEN Number");
    }
    #[test]
    fn found_even_is_success() {
        let output = matching_result(98);
        assert_eq!(output, "True => EVEN Number");
    }
}
