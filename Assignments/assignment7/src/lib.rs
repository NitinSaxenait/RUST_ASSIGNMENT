mod creating_substring;
mod pattern_matching;
mod ques3;
mod test;

// using unused_function to cover up the warning.
pub fn unused_function() {
    pattern_matching::pattern_match("as", "sa");
    ques3::compare_string("a", "a", "a");
    creating_substring::generate_substring("s");
}
