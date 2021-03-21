/// add_duplicate function is adding duplicates of given input list elements.
///
/// #Arguments
///
/// -> input_list is a i32 type list which contain all elements for which duplicates are to be added.
///
/// #Return
///
/// a Result type returning a Vec list containing list elements with duplicates or i32 for  empty list.
pub fn add_duplicate(input_list: &[i32]) -> Result<Vec<i32>, i32> {
    let mut collect_output: Vec<i32> = Vec::new();
    if input_list.is_empty() {
        return Err(0);
    }
    for index in input_list {
        collect_output.push(*index);
        collect_output.push(*index)
    }

    log::info!("List : {:?}", collect_output);
    Ok(collect_output)
}
