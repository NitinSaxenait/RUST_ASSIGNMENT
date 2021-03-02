use crate::Duplicate_Character_String::repeated_char;

mod Duplicate_Character_String;
mod Palindrome_orNot;
mod Rotation_String;

fn main() {
    let str = "Hello World";
    let i = 0;
    let j = i + 1;
    Duplicate_Character_String::repeated_char(&str, i, j);
    Palindrome_orNot::palindrome_true("nitin", 0, 0);
    Rotation_String::rotation_is_true("".to_string(), "".to_string());
}



