// checking_rotation function is checking whether a string1 is rotation of string2 or not.
//
// #Arguements
//
// input1_string is an input string which is going to reverse
// input2_string is an another string which is going to match to reverse of input1_string
//
// #Return
//
// return is going to return a boolean whether rotation is true or not
pub fn checking_rotation(input_string1: &str, input_string2: &str) -> bool {
    if input_string1.len() != input_string2.len() {
        return false;
    }
    input_string1.repeat(2).contains(&input_string2)
}
