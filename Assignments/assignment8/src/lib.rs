use crate::error_handling_on_evennumber::matching_result;

mod error_handling_on_evennumber;
mod test;
// used unused_function to manage the warnings for unused functions
pub fn unused_functions() {
    matching_result(55);
}
