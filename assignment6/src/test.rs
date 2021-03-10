#[cfg(test)]
//
// test cases for both the code
//
// 1.pattern_match=> for testing matching coordinates
// test
mod test {
    use crate::ques1_pattern_matching::Points;

    #[test]
    //
    // test_function-1
    //
    fn patter_match_test_paas() {
        let output1 = Points(-5, -8).find_coordinates();
        assert_eq!(
            output1,
            "Position::Third Quadrant(Coordinate::Abscissa -5, Coordinate::Ordinate -8 "
        );
    }
    //
    //test_function-2
    //
    #[test]
    fn pattern_pass() {
        let output = Points(2, 5).find_coordinates();
        assert_eq!(
            output,
            "Position::First Quadrant(Coordinate::Abscissa 2, Coordinate::Ordinate 5 "
        );
    }
    //
    // test_function 3
    //
    #[test]
    fn pattern_fail() {
        let output = Points(-2, 5).find_coordinates();
        assert_eq!(
            output,
            "Position::Second Quadrant(Coordinate::Abscissa -2, Coordinate::Ordinate 5 "
        );
    }
    //
    // test_function 4
    //
    #[test]
    fn pattern_pass_again() {
        let output = Points(2, -1).find_coordinates();
        assert_eq!(
            output,
            "Position::Fourth Quadrant(Coordinate::Abscissa 2, Coordinate::Ordinate -1 "
        );
    }
    //
    // test_function 5
    //
    #[test]
    fn test_for_origin() {
        let output = Points(0, 0).find_coordinates();
        assert_eq!(output, "0 and 0 lies on origin.")
    }

    // rest test cases are for Second Code
    //
    // for ques 2 => for ip address matching
    //
    // 4 test cases

    use crate::ques_2_create_enum::{match_input_ip, IpAddressClasses};

    #[test]
    //
    // test function 6
    //
    fn match_ip_true() {
        let output = match_input_ip(192, 0, 1, 1);
        assert_eq!(output, IpAddressClasses::ClassC(String::from("192.0.1.1")));
    }
    //
    // test_function 7
    //
    #[test]
    fn match_for_true() {
        let output = match_input_ip(230, 45, 6, 7);
        assert_eq!(output, IpAddressClasses::ClassD(String::from("230.45.6.7")));
    } //
      // test_function 8
      //
    #[test]
    fn match_for_true_again() {
        let output = match_input_ip(170, 45, 23, 45);
        assert_eq!(
            output,
            IpAddressClasses::ClassB(String::from("170.45.23.45"))
        );
    }
    //
    // test_function 9
    //
    #[test]
    fn ip_match_true() {
        let output = match_input_ip(198, 5, 6, 4);
        assert_eq!(output, IpAddressClasses::ClassC(String::from("198.5.6.4")));
    }
}
