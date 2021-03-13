//
// This function is handling error.
// For a taken_number is even or not.
//
// #Arguments
//
// Taken_number this argument is taken from other function arguments.
//
// #Return
//
// Handling_error function will return a Result type of (String,String).
pub fn handling_error(taken_number: i32) -> Result<String, String> {
    if taken_number % 2 == 0 {
        Ok("EVEN Number".to_string())
    } else {
        Err("NOT EVEN Number".to_string())
    }
}
// This function is matching the output from handling_error to the valid match.
//
// #Arguments
//
// input_number-> i32 type
//
// #Return
//
// This function return a String type for Ok or Err.
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
