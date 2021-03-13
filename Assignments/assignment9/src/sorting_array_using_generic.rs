// functions sort is used here for sorting process of any type list.
//
// function sort is generic here. <T>
// using PartialOrd for calculation of generic operands.
// and Display to display to output
//
// #Arguments
//
// -> generic type list which will take any type (character,integer or float).
//
// #Return
//
// returning a generic type sorted- > list

pub fn generic_sort<T: std::cmp::PartialOrd + std::fmt::Display>(input_list: &mut [T]) -> &mut [T] {
    for index in 0..input_list.len() {
        for count in index + 1..input_list.len() {
            if input_list[index] > input_list[count] {
                input_list.swap(index, count);
            }
        }
    }
    // returning generic type list
    input_list
}
