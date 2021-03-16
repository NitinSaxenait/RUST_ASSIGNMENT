/// function drop_from_vector is used to drop or remove a input value from the input vector
///
/// #Aguments
///
/// -> value as i32 is taken here amd a input list as Vec<i32> type
///
/// #Return
///
/// function is returning a vec<i32> type output as input_list
pub fn drop_from_list(value: i32, input_list: &mut Vec<i32>) -> &mut Vec<i32> {
    let some_x = value;
    let fomatted_list = input_list;
    fomatted_list.retain(|&x| x != some_x);
    fomatted_list
}
