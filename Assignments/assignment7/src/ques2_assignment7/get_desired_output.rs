use log::*;
/// Function -> comparing_string is comparing three input strings characters in sequence and returning the desired output.
///
/// Function will compare index of three string and return them according to small or big characters.
///
/// #Arguments
///
/// -> string1: string1 input one is used to compare with two and three.
/// -> string2: string2 input two is used to compare with one and three.
/// -> string3: string3 input three is used to compare with one and two.
///
/// #Return
///
/// a Option type returning a String as compared string or None if string 1,2,3 is empty.
pub fn comparing_string(string1: &str, string2: &str, string3: &str) -> Option<String> {
    if string1.is_empty() && string2.is_empty() && string3.is_empty() {
        warn!("All three strings are empty");
        return None;
    }
    let mut string_output: Vec<char> = Vec::new();
    let mut index = 0;
    let mut even_odd = 0;
    while index <= string1.len() {
        match even_odd % 2 == 0 {
            true => {
                let small_char = if string1.chars().nth(index) < string2.chars().nth(index) {
                    string1.chars().nth(index)
                } else {
                    string2.chars().nth(index)
                };
                let final_result = if small_char < string3.chars().nth(index) {
                    small_char
                } else {
                    string3.chars().nth(index)
                };
                if let Some(_p) = final_result {
                    string_output.push(final_result.unwrap());
                }
            }
            false => {
                let big_char = if string1.chars().nth(index) > string2.chars().nth(index) {
                    string1.chars().nth(index)
                } else {
                    string2.chars().nth(index)
                };
                let final_big_char = if big_char > string3.chars().nth(index) {
                    big_char
                } else {
                    string3.chars().nth(index)
                };
                if let Some(_p) = final_big_char {
                    string_output.push(final_big_char.unwrap());
                }
            }
        }
        even_odd += 1;
        index += 1;
    }
    let final_result: String = string_output.iter().collect();
    info!("The compared string is final result.");
    Some(final_result)
}
