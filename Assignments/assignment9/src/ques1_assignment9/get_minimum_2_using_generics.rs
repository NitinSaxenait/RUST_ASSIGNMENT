/// Function -> find_minimum is used here to find the minimum of two ->input generic value.
///
/// #Arguments
///
/// collect1 -> = generic value one collected from get_values function.
/// collect2 -> = generic value two collected from get_values function.
///
/// #Return
///
/// Function -> find_minimum_value is returning a -> generic type result as minimum value.
/// -> collect1 or -> collect2
pub fn find_minimum_value<'a, T: std::cmp::PartialOrd>(collect1: &'a T, collect2: &'a T) -> &'a T {
    if collect1 < collect2 {
        collect1
    } else {
        collect2
    }
}

/// Function get_values is getting user input and providing it to function -> find_minimum_value to evaluate minimum of two.
///
/// #Arguments
///
/// take_input1-> an int i32 type input as first value to compare.
/// take_input2-> an int i32 type input as second value to compare.
///
/// #Return
///
/// Function get_values returning a -> string type as the minimum value of two.

pub fn get_values(take_input1: i32, take_input2: i32) -> String {
    let got_output = find_minimum_value(&take_input1, &take_input2);
    let formatted_output = format!("The Minimum of two no is -> {}", got_output);
    formatted_output
}
