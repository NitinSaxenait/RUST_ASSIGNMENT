/// find_first_even function is finding the first even number from the given list of elements.
///
/// #Arguments
///
/// seq a i32 type list containing elements.
///
/// #Return
///
/// Option type containing Some even number or None for empty list.
pub fn find_first_even(seq: &[i32]) -> Option<i32> {
    let mut index = 0;
    let value;
    if seq.is_empty() {
        return None;
    }
    loop {
        if seq[index] % 2 == 0 {
            value = seq[index];
            break;
        }
        index += 1;
    }
    log::info!("List : {}", value);
    Some(value)
}
