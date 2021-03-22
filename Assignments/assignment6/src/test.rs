#[cfg(test)]

mod test {
    use crate::ques1_assignment6::ques1_pattern_matching::{find_coordinates, Axis, Coordinates};
    use crate::ques2_assignment6::ques_2_create_enum::{match_input_ip, IpAddressClasses};

    #[test]

    fn find_first_coordinates_success() {
        let output1 = (2, 3);
        assert_eq!(
            find_coordinates(output1),
            Axis::FirstQuadrant(Coordinates::Abscissa(2), Coordinates::Ordinate(3))
        );
    }
    #[test]
    fn find_second_coordinates_success() {
        let output1 = (-2, 3);
        assert_eq!(
            find_coordinates(output1),
            Axis::SecondQuadrant(Coordinates::Abscissa(-2), Coordinates::Ordinate(3))
        );
    }
    #[test]
    fn find_coordinates_origin_success() {
        let output1 = (0, 0);
        assert_eq!(
            find_coordinates(output1),
            Axis::Origin(Coordinates::Abscissa(0), Coordinates::Ordinate(0))
        );
    }
    #[test]
    fn find_coordinates_xaxis_success() {
        let output1 = (4, 0);
        assert_eq!(
            find_coordinates(output1),
            Axis::XAxis(Coordinates::Abscissa(4), Coordinates::Ordinate(0))
        );
    }
    #[test]
    fn find_coordinates_yaxis_success() {
        let output1 = (0, 9);
        assert_eq!(
            find_coordinates(output1),
            Axis::YAxis(Coordinates::Abscissa(0), Coordinates::Ordinate(9))
        );
    }
    #[test]
    fn find_coordinates_fourth_axis_success() {
        let output1 = (5, -9);
        assert_eq!(
            find_coordinates(output1),
            Axis::FourthQuadrant(Coordinates::Abscissa(5), Coordinates::Ordinate(-9))
        );
    }

    #[test]

    fn match_input_ip_class_a() {
        let output = match_input_ip(192, 0, 1, 1);
        assert_eq!(output, IpAddressClasses::ClassC(String::from("192.0.1.1")));
    }

    #[test]
    fn match_input_ip_class_d() {
        let output = match_input_ip(230, 45, 6, 7);
        assert_eq!(output, IpAddressClasses::ClassD(String::from("230.45.6.7")));
    }
    #[test]
    fn match_input_ip_class_b() {
        let output = match_input_ip(170, 45, 23, 45);
        assert_eq!(
            output,
            IpAddressClasses::ClassB(String::from("170.45.23.45"))
        );
    }

    #[test]
    fn match_input_ip_class_c() {
        let output = match_input_ip(198, 5, 6, 4);
        assert_eq!(output, IpAddressClasses::ClassC(String::from("198.5.6.4")));
    }
    #[test]
    fn match_input_ip_failure() {
        let output = match_input_ip(777, 877, 1000, 9898);
        assert_eq!(output, IpAddressClasses::None);
    }
}
