//
// check_palindrome function is taking input string and checking:
// whether string is making palindrome or not.
//
// #Arguments
//
// input_string-> a string type input used for palindrome.
// start_index -> its the starting point of the string index.
// end_index -> its the ending index of string.

// #Return
//
// function is returning a boolean function as true or false.

pub fn check_palindrome(input_string: &str, start_index: i32, end_index: i32) -> bool {
    if input_string.is_empty() {
        return true;
    }
    if start_index == end_index {
        return true;
    }
    if input_string.chars().nth(start_index as usize)
        != input_string.chars().nth(end_index as usize)
    {
        return false;
    }
    if start_index < end_index + 1 {
        return check_palindrome(input_string, start_index + 1, end_index - 1);
    }
    true
}
