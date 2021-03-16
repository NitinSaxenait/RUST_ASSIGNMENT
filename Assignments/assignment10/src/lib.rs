pub mod ques1_collection {
    use crate::ques1_collection::hasmap_::sum_conditional;

    pub mod hasmap_;
    pub fn unused_function() {
        sum_conditional(Default::default(), "a");
    }
}

mod test;
pub mod ques2_collection {
    use crate::ques2_collection::drop_from_vector::drop_from_list;
    use crate::ques2_collection::first_even_number::first_even;
    use crate::ques2_collection::is_palindrome::if_palindrome;
    use crate::ques2_collection::remove_continous_sequence::remove_continous_value;
    use crate::ques2_collection::reverse_list::make_reverse;

    pub mod add_duplicate;
    pub mod drop_from_vector;
    pub mod first_even_number;
    pub mod is_palindrome;
    pub mod remove_continous_sequence;
    pub mod reverse_list;
    pub fn unused_function() {
        make_reverse(&[1]);
        remove_continous_value(&mut vec![1]);
        first_even(&[1]);
        drop_from_list(0, &mut vec![1]);
        if_palindrome(&[1]);
        add_duplicate::add_duplicate(&[1]);
    }
}
