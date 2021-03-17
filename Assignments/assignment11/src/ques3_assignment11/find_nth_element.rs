use crate::list::List;
use crate::list::List::{Cons, Nil};


/// Function find_nth_repeated find the nth repeated number from input_list.
///
/// #Arguments
///
/// -> input_index : an enum type which contain an input index to locate nth value.
///
/// #Return
///
/// Function returning the -> i32 type that contain nth repeated number from list.

pub fn find_nth_repeated(input_index: i32, input_list: List) -> i32 {
    continous(input_index - 1, input_list, 0)
}

/// Function continous use recursion to match list object and find nth number.
///
/// #Arguments
///
/// input_index - An i32 variable containing the position of number to find in list.
///
/// take_list- A List enum object which contains the list of numbers.
///
/// 'counter' - An i32 variable containing the position of current number in list.
///
/// #Return
///
/// Return the i32 number containing nth number.

pub fn continous(take_index: i32, take_list: List, counter: i32) -> i32 {
    match take_list {
        Nil => -1,
        Cons(initial, _) if counter == take_index => initial,
        Cons(_, take_list) => continous(take_index, *take_list, counter + 1),
    }
}
