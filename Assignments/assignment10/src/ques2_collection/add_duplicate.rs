///
/// add_duplicate function is adding duplicates of given input list elements.
/// as 1->11, 2,2-> 2,2,2,2
///
/// #Arguments
///
/// -> input_list of Vec<i32> type is used here which contain all i32 types values.
///
/// #Return
///
/// Function add_duplicates is returning a Vec<i32> type vector list which have all duplicate values.
///
/// Use logger to pass output as collect_output.
pub fn add_duplicate(input_list: &[i32]) -> Vec<i32> {
    let mut collect_output: Vec<i32> = Vec::new();

    for index in input_list {
        collect_output.push(*index);
        collect_output.push(*index)
    }

    log::info!("List : {:?}", collect_output);
    collect_output
}
