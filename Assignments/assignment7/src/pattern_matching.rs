//
// this function is used to match pattern on a string.
// #Arguments
//
// string:str a input string which will contain the pattern.
// pattern:str a input pattern which will match on string.
//
// #Return
//
// String: function will will return a string;
// if match found : return pattern found at index or else return pattern out of bound
pub fn pattern_match(string: &str, pattern: &str) -> String {
    let collect_string: Vec<char> = string.chars().collect();
    let collect_pattern: Vec<char> = pattern.chars().collect();
    let mut count = 0;
    let mut take_index_value;
    // loop iterate from index 0 to difference of both string and pattern
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
