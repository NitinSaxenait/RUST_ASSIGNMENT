use crate::min_of_two_using_generic::{find_minimum, minimum_maximum};
use crate::sorting_array_using_generic::generic_sort;

mod custom_iterator_using_generic;
mod min_of_two_using_generic;
mod sorting_array_using_generic;
mod test;

// unused_function used to control the warnings for unused function.
pub fn unused_functions() {
    find_minimum(&2, &5);
    minimum_maximum(5, 7);
    generic_sort(&mut [1, 2]);
}
