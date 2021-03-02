pub fn palindrome_true(str: &str, beg: i32, last: i32) -> bool {
    //for single character(first and last value will be same)
    if beg == last {
        return true;
    }

    //return false if first and last character is different
    //if condition matching first and last character of a string which is different
    // returning false
    if str.chars().nth(beg as usize) != str.chars().nth(last as usize) {
        return false;
    }


    //for multiple character check if middle substring is palindrome or not
    if beg < last + 1 {
        return palindrome_true(str, beg + 1, last - 1);
    }
    //hence returning true
    return true;
}
