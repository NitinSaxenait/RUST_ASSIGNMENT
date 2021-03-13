// function find_minimum is used here to find the minimum of two generic value.
//
// used generic can take any type value work as a super set.
//
// #Arguments
//
// collect1-> = generic value one collected from minimum_maximum.
// collect2-> = generic value two collected from minimum_maximum.
//
// #Return
//
// function find_minimum will return a generic type result as (collect1 or collect2).
pub fn find_minimum<'a, T: std::cmp::PartialOrd>(collect1: &'a T, collect2: &'a T) -> &'a T {
    if collect1 < collect2 {
        collect1
    } else {
        collect2
    }
}

// function minimum_maximum is getting user input and providing it to find_minimum function
//
// #Arguments
//
// input1-> an int i32 type input as 1st value.
// input2-> an int i32 type input as 2nd value.
//
// #Return
//
// function minimum_maximum returning a string type as output.
// the minimum of two no is {}.

pub fn minimum_maximum(input1: i32, input2: i32) -> String {
    let got_output = find_minimum(&input1, &input2);
    let formatted_output = format!("The Minimum of two no is -> {}", got_output);
    // returning String ->formatted_output.
    formatted_output
}
