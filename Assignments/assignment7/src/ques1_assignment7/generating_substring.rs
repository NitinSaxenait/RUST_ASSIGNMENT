/// Function -> generate_substring is (generating subsets) of a -> input_string.
///
/// In form  => ap -> a , p, ap
///
/// #Arguments
///
/// -> input_string: A String whose subsets is to be made.
///
/// #Return
///
/// Function will return a -> vector_String of string's subsets.
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
