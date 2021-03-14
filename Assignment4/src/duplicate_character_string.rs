// count_same_characters is used to find the duplicate characters(Repeatble characters) in a string.
//
// #Arguements
//
// -> input_string: is takes as a argument in which duplication is checked.
//
// #Return
// 
// -> String:
//
// function is going to return a String type as a final_output.
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
        final_output;
    }


// test cases for repeated characters
//
// test cases
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn duplicate_is_true() {
        assert_eq!(count_same_characters("hello"), "true")
    }
    #[test]
    fn duplicate_true() {
        assert_eq!(count_same_characters("apple"), "true")
    }
    #[test]
    fn duplicate_false() {
        assert_eq!(count_same_characters("samuel"), "false")
    }
}
