use crate::list::List;
use crate::list::List::{Cons, Nil};
use log::info;

/// Function -> find_third_odd find the third odd repeated number from input_list.
///
/// #Arguments
///
/// -> input_list : an enum type which contain list of numbers.
///
/// #Return
///
/// Function returning the -> i32 type that contain third odd number from list.

pub fn find_third_odd(input_list: List) -> i32 {
    continous(3, input_list)
}

/// continous function use recursion to match list values and find third odd value/element.
///
/// #Arguments
///
/// 'counter' - An i32 variable containing the count of initial odd number in list.
///
/// take_list contain is enum type contains all elements.
///
/// #Return
///
/// Function return the i32 type number that give third odd number.

pub fn continous(counter: i32, take_list: List) -> i32 {
    info!("Find third odd number");
    match take_list {
        Nil => -1,
        Cons(initial, _) if counter == 1 && initial % 2 != 0 => initial,
        Cons(initial, take_list) if initial % 2 != 0 => continous(counter - 1, *take_list),
        Cons(_, take_list) => continous(counter, *take_list),
    }
}
