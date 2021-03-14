// code to find the repeated character in a string
//
// #Arguements
//
// a string is taken as a parameter to verify whether it has duplicate char or not
//
// #Return
//
// return a string a final output
pub fn count_same_characters(input_string: &str) -> String {
    let mut vector_chars: Vec<char> = input_string.chars().collect();
    let mut initial_index = 0;
    let mut final_output = String::new();
    while initial_index < input_string.len() {
        let mut counter = 1;
        let mut next_index = initial_index + 1;
        while next_index < input_string.len() {
            if vector_chars[initial_index] == vector_chars[next_index]
                && vector_chars[initial_index] != ' '
            {
                counter += 1;
                vector_chars[next_index] = '0';
            }
            next_index += 1;
        }
        if counter > 1 && vector_chars[initial_index] != '0' {
            final_output.push(vector_chars[initial_index]);
        }
        initial_index += 1;
    }
    final_output
}
