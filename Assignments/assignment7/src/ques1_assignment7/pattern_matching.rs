/// Function match_pattern is used to match (input pattern) over (input string).
///
/// #Arguments
///
/// -> string: str a input string on which pattern is going to match.
/// -> pattern: str a input pattern which is to be matched on a input_string.
///
/// #Return
///
/// match_pattern function will return a -> String of "pattern matched at index or not matched".
pub fn match_pattern(string: &str, pattern: &str) -> String {
    let collect_string: Vec<char> = string.chars().collect();
    let collect_pattern: Vec<char> = pattern.chars().collect();
    let mut count = 0;
    let mut take_index_value;

    for index in 0..(collect_string.len() - collect_pattern.len() + 1) {
        take_index_value = index;

        for travel in &collect_pattern {
            if travel == &collect_string[take_index_value] {
                count += 1;
            }
            if count == collect_pattern.len() {
                return format!("pattern found at index {}", index);
            }
            take_index_value += 1;
        }
        count = 0;
    }
    "pattern is not present in given string".to_string()
}
