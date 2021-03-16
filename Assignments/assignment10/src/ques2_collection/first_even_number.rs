///
/// first_even function is only taking the first even number from the given list of sequence.
/// checking the first even number using the even condition for every element until it found the first even element
///
/// #Arguments
///
/// -> input_list as a Vector<i32> type list of elements.
///
/// #Return
///
/// function first_even is returning a i32 type value as first even number from the given list of sequence.
///
/// Use logger to collect value.
pub fn first_even(seq: &[i32]) -> i32 {
    let mut index = 0;
    let value;
    loop {
        if seq[index] % 2 == 0 {
            value = seq[index];
            break;
        }
        index += 1;
    }
    log::info!("List : {}", value);
    value
}
