///
/// if_palindrome function is taking input and checking if the given input_list  is making palindrome or not.
///
/// #Arguments
///
///  input_list -> a i32 type vector list is used as a input
///
/// #Return
///
/// function is returning a -> String type for Makes Palindrome or No.

pub fn if_palindrome(input_list: &[i32]) -> String {
    let mut list_length = input_list.len();

    for index in 0..input_list.len() {
        let start = input_list[index];
        list_length -= 1;
        let end = input_list[list_length];

        if start != end {
            String::from("No");
            break;
        }
    }

    String::from("Makes Palindrome")
}
