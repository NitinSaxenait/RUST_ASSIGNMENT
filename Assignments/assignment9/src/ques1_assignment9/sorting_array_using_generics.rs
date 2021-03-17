/// Functions -> generic_sorting is used here for sorting any type (generic type) ->list.
///
/// Function generic_sorting is -> generic here. <T>
///
/// #Arguments
///
/// -> input_list : a generic list which will take any type (character,integer or float).
///
/// #Return
///
/// Function generic_sorting is returning a generic type sorted list.

pub fn generic_sorting<T: std::cmp::PartialOrd + std::fmt::Display>(
    input_list: &mut [T],
) -> &mut [T] {
    for index in 0..input_list.len() {
        for count in index + 1..input_list.len() {
            if input_list[index] > input_list[count] {
                input_list.swap(index, count);
            }
        }
    }
    input_list
}
