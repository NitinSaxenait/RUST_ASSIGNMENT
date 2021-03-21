/// remove_continous_vales function is removing all duplicate value coming in sequence of list.
///
/// #Arguments
///
/// input_list a list containing i32 type elements.
///
/// #Return
///
/// returning an Option type for as new list or None for empty input list.
pub fn remove_continous_value(input_list: &mut Vec<i32>) -> Option<Vec<i32>> {
    let length = input_list.len();
    let mut collect: Vec<i32> = Vec::new();
    if input_list.is_empty() {
        return None;
    }
    for index in 0..input_list.len() - 1 {
        if input_list[index] != input_list[index + 1] {
            collect.push(input_list[index]);
        }
    }
    collect.push(input_list[length - 1]);
    log::info!("List : {:?}", collect);
    Some(collect)
}
