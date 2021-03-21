/// make_reverse function is used to reverse a list elements.
///
/// #Arguments
///
/// input_list a list containing i32 type elements which are to be reversed.
///
/// #Return
///
/// function is returning a reverse list or String as Empty list.

pub fn make_reverse(input_list: &[i32]) -> Result<Vec<i32>, String> {
    if input_list.is_empty() {
        return Err(String::from("Empty list"));
    }
    let mut list_length = input_list.len();
    let mut reverse_collector: Vec<i32> = Vec::new();
    for _index in 0..input_list.len() {
        list_length -= 1;
        reverse_collector.push(input_list[list_length]);
    }

    Ok(reverse_collector)
}
