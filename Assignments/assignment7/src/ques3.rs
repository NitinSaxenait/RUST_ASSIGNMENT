//
// this function is comparing strings character according to their order.
//
// this will compare index of three string and return them according to small or big char.
// using match function it will iterate for even and odd value
//
// for true return small character
// for false return big character

// #Arguments
//
// [string 1, string 2, string 3] will be the arguments for this function.
//
// #Return
//
// function will return a String as final result
pub fn compare_string(string1: &str, string2: &str, string3: &str) -> String {
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
    // returning String as final_result.
    final_result
}
