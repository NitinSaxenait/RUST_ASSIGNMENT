/// Function -> handling_error is used here to handle error on -> taken_number from function -> matching_result.
/// To handle error : Whether taken_number is even or not.
///
/// #Arguments
///
/// -> taken_number : a i32 type is taken as argument to check if even or not.
///
/// #Return
///
/// Function handling_error is returning a Result<String,String>type  for a number is even or not even.
fn handling_error(taken_number: i32) -> Result<String, String> {
    if taken_number % 2 == 0 {
        Ok("EVEN Number".to_string())
    } else {
        Err("NOT EVEN Number".to_string())
    }
}
/// Function matching_result is matching the -> output of function -> handling_error to Okk or Err.
///
/// #Arguments
///
/// -> input_number :i32 type input is taken to check if input_number is even or not.
///
/// #Return
///
/// This function return a -> String of Ok or Err for (true or error).
pub fn matching_result(input_number: i32) -> String {
    let output = handling_error(input_number);
    match output {
        Ok(_true) => {
            let get_it = format!("True => {}", _true);
            get_it
        }
        Err(e) => e,
    }
}
