mod ques1_pattern_matching;
mod ques_2_create_enum;
mod test;

use crate::ques1_pattern_matching::Points;

//
// function unwanted is used to remove warnings from the program.
pub fn unwanted() {
    ques_2_create_enum::match_input_ip(2, 5, 4, 4);
    ques1_pattern_matching::Points::find_coordinates(&Points(2, 3));
}
