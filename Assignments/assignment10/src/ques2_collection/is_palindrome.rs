/// if_palindrome function is taking input list and checking if it is making palindrome or not.
///
/// #Arguments
///
///  input_list -> a list containing i32 type elements.
///
/// #Return
///
/// Result type returning string for palindrome and string for no palindrome.

pub fn if_palindrome(input_list: &[i32]) -> Result<String, String> {
    let mut list_length = input_list.len();

    for index in 0..input_list.len() {
        let start = input_list[index];
        list_length -= 1;
        let end = input_list[list_length];

        if start != end {
            return Err(String::from("No"));
        }
    }

    Ok(String::from("Makes Palindrome"))
}
