//program to check if string is reverse of string 2
//
//function is checking if rotation is true or not
//
//returning string as output
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

//test cases to check rotation functions
//
//4 test functions are used
//all passed the test cases
#[cfg(test)]
use super::*;
mod test {
    use super::*;

    #[test]
    fn test1_reverse() {
        assert_eq!(reverse_is_true("ram", "mar"), "rotation is true")
    }
    #[test]
    fn test2_for_reverse() {
        assert_eq!(reverse_is_true("rahul", "rama"), "rotation is false")
    }
    #[test]
    fn test3_for_reverse() {
        assert_eq!(reverse_is_true("abcdefg", "gfedcba"), "rotation is true")
    }
    #[test]
    fn test4_for_reverse() {
        assert_eq!(reverse_is_true("nitin", "nitin"), "rotation is true")
    }
}
