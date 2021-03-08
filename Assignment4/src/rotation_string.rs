// this function is used to reverse a string
//
// #Arguements
//
// input1_string is an input string which is going to reverse
// input2_string is an another string which is going to match to reverse of input1_string
//
// #Return
//
// return is going to return a string whether rotation is true or not

pub fn reverse_is_true(input1_string: &str, input2_string: &str) -> String {
    let mut collect_reverse_char = String::new();
    let mut output = String::new();
    for count in input1_string.chars().rev() {
        collect_reverse_char.push(count);
    }
    if collect_reverse_char == input2_string {
        output.push_str("rotation is true");
    } else {
        output.push_str("rotation is false");
    }
    output
}

// test cases for reverse is true function
//
// test cases
#[cfg(test)]
use super::*;
mod test {
    use super::*;

    #[test]
    fn rotation_true() {
        assert_eq!(reverse_is_true("ram", "mar"), "rotation is true")
    }
    #[test]
    fn rotation_false() {
        assert_eq!(reverse_is_true("rahul", "rama"), "rotation is false")
    }
    #[test]
    fn rotation__is_true() {
        assert_eq!(reverse_is_true("abcdefg", "gfedcba"), "rotation is true")
    }
    #[test]
    fn rotation_is_true() {
        assert_eq!(reverse_is_true("nitin", "nitin"), "rotation is true")
    }
}
