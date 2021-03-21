use regex::Regex;
use std::collections::HashMap;
/// Function sum_conditional calculate the age of persons.
///
/// #Arguments
///
/// -> input_map is used to take string type value and a i32 type value.
/// -> string  : type is used here to provide the pattern to match.
///
/// #Return
///
/// function is returning a Option as i32 value.
pub fn sum_conditional(input_map: HashMap<&str, i32>, string: &str) -> Option<i32> {
    let mut collect_value = 0;
    if input_map.is_empty() {
        return None;
    }
    let new_regrex = Regex::new(&*(r"".to_owned() + string)).unwrap();
    for person in input_map {
        if new_regrex.is_match(person.0) {
            collect_value += person.1
        }
    }

    Some(collect_value)
}
