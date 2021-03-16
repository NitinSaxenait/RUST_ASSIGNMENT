///
/// make_reverse function is used to reverse a given input_list vec<i32> type
/// a reverse collector vec<i32> type is used to collect all the reverse values of list.
///
/// #Arguments
///
/// -> input_list is used as a input for the function which consists all i32 type values in list.
///
/// #Return
///
/// Function is returning a Vec<i32> type.

pub fn make_reverse(input_list: &[i32]) -> Vec<i32> {
    let mut list_length = input_list.len();
    let mut reverse_collector: Vec<i32> = Vec::new();
    for _index in 0..input_list.len() {
        list_length -= 1;
        reverse_collector.push(input_list[list_length]);
    }

    reverse_collector
}
