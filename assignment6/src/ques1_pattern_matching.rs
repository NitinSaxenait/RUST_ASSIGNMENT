pub struct Points(pub i32, pub i32);
// Quadrants structure providing variants
//
// first_quadrant, second_quadrant, third quadrant, fourth quadrant as String.
//
#[derive(PartialEq, Eq, Debug)]
struct Quadrants {
    first_quadrant: String,
    second_quadrant: String,
    third_quadrant: String,
    fourth_quadrant: String,
}
//
// code is used to match the input co-ordinates
//
// #Arguments
//
// input_x as first coordinate point.
// input_y as second coordinate point.
//
// #Return
//
// String value as output.
impl Points {
    pub fn find_coordinates(&self) -> String {
        let input_x = self.0;
        let input_y = self.1;
        let take = Quadrants {
            first_quadrant: String::from("First Quadrant"),
            second_quadrant: String::from("Second Quadrant"),
            third_quadrant: String::from("Third Quadrant"),
            fourth_quadrant: String::from("Fourth Quadrant"),
        };
        // this is used to match the given point to certain conditions.
        match (input_x, input_y) {
            //
            // condition for First Quadrant
            //
            (input_x, input_y) if input_x > 0 && input_y > 0 => format!(
                "Position::{}(Coordinate::Abscissa {}, Coordinate::Ordinate {} ",
                take.first_quadrant, input_x, input_y
            ),
            //
            // condition for Second Quadrant
            //
            (input_x, input_y) if input_x < 0 && input_y > 0 => format!(
                "Position::{}(Coordinate::Abscissa {}, Coordinate::Ordinate {} ",
                take.second_quadrant, input_x, input_y
            ),
            //
            // condition for Third Quadrant
            //
            (input_x, input_y) if input_x < 0 && input_y < 0 => format!(
                "Position::{}(Coordinate::Abscissa {}, Coordinate::Ordinate {} ",
                take.third_quadrant, input_x, input_y
            ),
            //
            // condition for Fourth Quadrant
            //
            (input_x, input_y) if input_x > 0 && input_y < 0 => format!(
                "Position::{}(Coordinate::Abscissa {}, Coordinate::Ordinate {} ",
                take.fourth_quadrant, input_x, input_y
            ),
            _ => format!("{} and {} lies on origin.", input_x, input_y),
        }
    }
}
