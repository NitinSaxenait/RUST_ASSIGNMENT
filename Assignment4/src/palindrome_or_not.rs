//program to check whether string is making palindrome or not
//
//function taking two string as parameter as string1 and string2
//
//function to check palindrome condition
fn palindrome_is_true(input: &str, string1_length: usize, string2_length: usize) -> bool {
    //if string is having only single character
    //return true as it will made a palindrome

    if string1_length == string2_length {
        return true;
    }

    // If first and last
    // characters do not match
    if input.as_bytes()[string1_length] != input.as_bytes()[string2_length] {
        return false;
    }

    // If string 1 is less then string 2 then
    //check whether it make a palindrome or not
    //return true is it make a palindrome
    if string1_length < string2_length + 1 {
        return palindrome_is_true(input, string1_length + 1, string2_length - 1);
    }
    return true;
}

//function checking whether given string making palindrome or not
//return string
pub fn check_for_palindrome(input: &str) -> String {
    let string_length: usize = input.len();
    let is_palindrome = palindrome_is_true(input, 0, string_length - 1);

    let mut result = String::new();
    //if condition matches
    //return true for that condition
    if is_palindrome == true {
        result.push_str("Palindrome");
    } else {
        result.push_str("Not Palindrome");
    }
    result
}

//test stage to verify palindrome for given string
//
//for string "abba"
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_palindrome1() {
        assert_eq!(check_for_palindrome("abba"), "Palindrome");
    }
    #[test]
    fn check_palindrome2() {
        assert_eq!(check_for_palindrome("sass"), "Not Palindrome");
    }
    #[test]
    fn check_palindrome3() {
        assert_eq!(check_for_palindrome("Samar"), "Not Palindrome");
    }
    #[test]
    fn check_palindrome4() {
        assert_eq!(check_for_palindrome("kllk"), "Palindrome");
    }
}
