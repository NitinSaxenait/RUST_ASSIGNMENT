use crate::list::List;
use crate::list::List::{Cons, Nil};
use log::info;

/// Function find_first_repeated find the second repeated number from input_list.
///
/// #Arguments
///
/// -> input_list : an enum type which contain list of numbers
///
/// #Return
///
/// Function returning the -> i32 type that contain second repeated number from list.

pub fn find_second_repeated(input_list: List) -> i32 {
    continous(-1, input_list, 0)
}

/// continous function use recursion to match list values and find second repeated value/element.
///
/// #Arguments
///
/// "previous_value" is a i32 type taking value from find_second_repeated.
///
/// take_list contain is enum type contains all elements.
///
/// #Return
///
/// Function return the i32 type number that give second repeated number.

pub fn continous(previous_value: i32, take_list: List, counter: i32) -> i32 {
    info!("Find second repeated number");
    match take_list {
        Nil => -1,
        Cons(initial, _) if initial == previous_value && counter == 1 => initial,
        Cons(initial, take_list) if initial == previous_value => {
            continous(initial, *take_list, counter + 1)
        }
        Cons(initial, take_list) => continous(initial, *take_list, counter),
    }
}
