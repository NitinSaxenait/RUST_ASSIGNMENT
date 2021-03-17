mod testing {
   pub use crate::list::List::{Cons, Nil};
    pub use crate::ques1_assignment11::first_repeated_number::find_first_repeated;
    pub use crate::ques2_assignment11::second_repeated_number::find_second_repeated;
    pub use crate::ques3_assignment11::find_nth_element::find_nth_repeated;
    pub use crate::ques4_assignment11::find_third_odd::find_third_odd;

    #[test]
    fn find_first_repeated_success() {
        env_logger::init();
        let test_list = Cons(
            1,
            Box::new(Cons(
                2,
                Box::new(Cons(
                    2,
                    Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(4, Box::new(Nil))))))),
                )),
            )),
        );
        assert_eq!(find_first_repeated(test_list), 2);
    }
    #[test]
    fn find_first_repeated_failure() {
        let test_list = Cons(
            1,
            Box::new(Cons(
                2,
                Box::new(Cons(
                    3,
                    Box::new(Cons(4, Box::new(Cons(5, Box::new(Cons(6, Box::new(Nil))))))),
                )),
            )),
        );
        assert_eq!(find_first_repeated(test_list), -1);
    }
    #[test]
    fn find_second_repeated_success() {
        let test_list = Cons(
            1,
            Box::new(Cons(
                2,
                Box::new(Cons(
                    2,
                    Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(4, Box::new(Nil))))))),
                )),
            )),
        );
        assert_eq!(find_second_repeated(test_list), 4);
    }
    #[test]
    fn find_second_repeated_failure() {
        let test_list = Cons(
            1,
            Box::new(Cons(
                2,
                Box::new(Cons(
                    2,
                    Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))),
                )),
            )),
        );
        assert_eq!(find_second_repeated(test_list), -1);
    }
    #[test]
    fn find_nth_value_check() {
        let test_list = Cons(
            1,
            Box::new(Cons(
                2,
                Box::new(Cons(
                    2,
                    Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(4, Box::new(Nil))))))),
                )),
            )),
        );
        assert_eq!(find_nth_repeated(4, test_list), 3);
    }

    #[test]
    fn find_third_odd_success() {
        let test_list = Cons(
            1,
            Box::new(Cons(
                21,
                Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))),
            )),
        );
        assert_eq!(find_third_odd(test_list), 3);
    }
    #[test]
    fn find_third_odd_failure() {
        let test_list = Cons(
            1,
            Box::new(Cons(
                21,
                Box::new(Cons(4, Box::new(Cons(4, Box::new(Cons(6, Box::new(Nil))))))),
            )),
        );
        assert_eq!(find_third_odd(test_list), -1);
    }
}
