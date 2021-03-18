#[cfg(test)]

mod test {
    use crate::ques1_assignment12::asynchronous_table::block;
    use crate::ques2_assignment12::async_function::mad;
    #[test]
    fn asynchronous_table_success() {
        assert_eq!(block(), ());
    }
    #[test]
    fn async_function_success() {
        assert_eq!(mad(), ())
    }
}
