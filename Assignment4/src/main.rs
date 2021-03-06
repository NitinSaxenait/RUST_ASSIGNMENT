mod duplicate_character_string;
mod palindrome_or_not;
mod rotation_string;

//main function verifying all the programs
fn main() {
    duplicate_character_string::repeated_characters::count_same_characters("hello");
    rotation_string::reverse_is_true("ram", "mar");
    let variable = "nitin";
    println!(
        "String is : {}",
        palindrome_or_not::check_for_palindrome(variable)
    );
}
