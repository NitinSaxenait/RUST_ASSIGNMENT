use crate::list::List;
use crate::list::List::{Cons, Nil};
use log::info;

/// Function find_first_repeated find the first repeated number from input_list.
///
/// #Arguments
///
/// -> input_list : an enum type which contain list of numbers
///
/// #Return
///
/// Function returning the -> i32 type that contain first repeated number from list.

pub fn find_first_repeated(input_list: List) -> i32 {
    continous(-1, input_list)
}

/// continous function use recursion to match list values and find first repeated value/element.
///
/// #Arguments
///
/// "previous_value" is a i32 type taking value from find_first_repeated.
///
/// take_list contain is enum type contains all elements.
///
/// #Return
///
/// Function return the i32 type number that give first repeated number.

pub fn continous(previous_value: i32, take_list: List) -> i32 {
    info!("first repeated number");
    match take_list {
        Nil => -1,
        Cons(first_value, _) if first_value == previous_value => first_value,
        Cons(first_value, take_list) => continous(first_value, *take_list),
    }
}
