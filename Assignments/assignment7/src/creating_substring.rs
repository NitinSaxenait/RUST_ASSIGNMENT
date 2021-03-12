//
// this function is generating subsets of a string.
//
// using two loops for that first for first index and second loop for index+1.
//
// #Arguments
//
// taking input as 'input_string".
//
// #Return
//
// function will return a vector string as "take_result.
pub fn generate_substring(input_string: &str) -> Vec<String> {
    let string_vector: Vec<char> = input_string.chars().collect();
    let mut take_count;
    let mut take_travel;
    let mut take_result: Vec<String> = Vec::new();
    let mut concatenate: String;
    take_result.push(input_string.parse().unwrap());

    for count in 0..(input_string.len()) {
        take_result.push(string_vector[count].to_string());

        for travel in count + 1..input_string.len() {
            take_count = string_vector[count].to_string();
            take_travel = string_vector[travel].to_string();
            concatenate = take_count + &*take_travel;
            take_result.push(concatenate);
        }
    }

    take_result
}
