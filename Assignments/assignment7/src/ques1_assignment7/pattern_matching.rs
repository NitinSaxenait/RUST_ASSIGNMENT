use log::*;
/// Function match_pattern is used to match (input pattern) over (input string).
///
/// #Arguments
///
/// -> string: str a input string on which pattern is going to match.
/// -> pattern: str a input pattern which is to be matched on a input_string.
///
/// #Return
///
/// a Result returning a string type as pattern found at index or pattern not found.
pub fn match_pattern(string: &str, pattern: &str) -> Result<String, String> {
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
                info!("Pattern found successfully");
                return Ok(format!("pattern found at index {}", index));
            }
            take_index_value += 1;
        }
        count = 0;
    }
    warn!("Pattern not found");
    Err("pattern is not present in given string".to_string())
}
