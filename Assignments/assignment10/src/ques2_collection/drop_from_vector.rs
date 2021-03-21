/// function drop_from_vector is used to drop a input value from input_list.
///
/// #Aguments
///
/// value -> a i32 type which is to be removed.
/// input_list -> a i32 type list from which value is to be drop.
///
/// #Return
///
/// function is returning a Result type as new list or string for empty input_list.
pub fn drop_from_list(value: i32, input_list: &mut Vec<i32>) -> Result<&mut Vec<i32>, String> {
    let some_x = value;
    if input_list.is_empty() {
        return Err(String::from("Sorry,your list is empty."));
    }
    let new_list = input_list;
    new_list.retain(|&x| x != some_x);
    Ok(new_list)
}
