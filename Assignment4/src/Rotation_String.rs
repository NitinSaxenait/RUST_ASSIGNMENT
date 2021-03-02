//program to check if two string are rotation of each other
pub fn rotation_is_true(str1: String, str2: String) -> bool {
    let mut str_len1 = str1.len();
    let mut str_len2 = str2.len();
    let mut string1: Vec<char> = str1.chars().collect();
    let mut string2: Vec<char> = str2.chars().collect();
    if str_len1 != str_len2 {
        return false;
    }
    let mut longest_prefix_suffix = Vec::with_capacity(str_len1);

    let mut prev_len = 0;
    let mut count = 1;


    longest_prefix_suffix[0] = 0;

//loop checking if count vale is less then length 1
    while count < str_len1 {
        //if length count is equal to previous length
        if string1[count] == string2[prev_len] {
            longest_prefix_suffix[count] = prev_len + 1;
            count = count + 1;
        } else {
            if prev_len == 0 {
                longest_prefix_suffix[count] = 0;
                count = count + 1;
            } else {
                prev_len = longest_prefix_suffix[prev_len - 1];
            }
        }

    }

    count = 0;


    let mut k = longest_prefix_suffix[str_len1 - 1];
    while k < str_len2 {
        if string2[k] != string1[count] {
            count = count + 1;
            return false;
        }
        k = k + 1;
    }
    return true;
}