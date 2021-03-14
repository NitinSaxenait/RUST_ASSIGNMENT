// check_for_rotation is going to check if a input_string1 is rotation of input2_string.
// 
// #Arguments
// 
// input_string1 -> a string type input one.
// input_string2 -> a string type input two.
//
// #Return
// 
// function check_for rotation will return a boolean type as true or false.
pub fn check_for_rotation(input_string1: &str,input_string2: &str) -> bool {
    if input_string1.len() != input_string2.len() {
        return false;
    }
    input_string1.repeat(2).contains(&input_string2)
}
//
// test cases for check_for_rotation function.
//
#[cfg(test)]
use super::*;
mod test {
    use super::*;

    #[test]
    fn rotation_true() {
        assert_eq!(check_for_rotation("ram", "mar"),true)
    }
    #[test]
    fn rotation_false() {
        assert_eq!(check_for_rotation("rahul", "rama"),false)
    }
    #[test]
    fn rotation__is_true() {
        assert_eq!(check_for_rotation("abcdefg", "gfedcba"),true)
    }
    #[test]
    fn rotation_is_true() {
        assert_eq!(check_for_rotation("nitin", "nitin"),true)
    }
}
