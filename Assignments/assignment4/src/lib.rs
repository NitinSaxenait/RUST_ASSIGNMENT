use crate::check_palindrome::check_palindrome;
use crate::find_duplicate_charaters::count_same_characters;
use crate::rotation_is_true::checking_rotation;
mod check_palindrome;
mod find_duplicate_charaters;
mod rotation_is_true;
mod test;

pub fn unused_functions() {
    count_same_characters("s");
    checking_rotation("a", "d");
    check_palindrome("a", 0, 4);
}
