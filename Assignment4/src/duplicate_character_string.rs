//program to find duplicate characters in string
//
pub mod repeated_characters {
    //function taking string as parameter
    //checking if string have any same character or not
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
        return final_output;
    }
}

//test cases for the given function lies here
//
//3 test cases are tested in this function with different values
#[cfg(test)]
mod test {
    use crate::duplicate_character_string::repeated_characters::count_same_characters;
    #[test]
    fn testcase1() {
        assert_eq!(count_same_characters("hello"), "true")
    }
    #[test]
    fn testcase2() {
        assert_eq!(count_same_characters("apple"), "true")
    }
    #[test]
    fn testcase3() {
        assert_eq!(count_same_characters("samuel"), "false")
    }
}
