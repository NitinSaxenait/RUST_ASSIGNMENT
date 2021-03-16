///
/// Remove_continous_value function is removing all duplicate value coming in sequence of list.
///
/// A collect vec i32 is also used to collect all the value of list.
///
/// #Arguments
///
/// -> input_list a Vec i32 type list is a input for the function.
///
/// #Return
///
/// Function is returning a -> Vec<i32> type list with all single value collection
///
/// Use info logger to collect the output as collect
pub fn remove_continous_value(input_list: &mut Vec<i32>) -> Vec<i32> {
    let length = input_list.len();
    let mut collect: Vec<i32> = Vec::new();
    for index in 0..input_list.len() - 1 {
        if input_list[index] != input_list[index + 1] {
            collect.push(input_list[index]);
        }
    }
    collect.push(input_list[length - 1]);

    log::info!("List : {:?}", collect);
    collect
}
