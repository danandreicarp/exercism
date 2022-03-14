use core::fmt::Display;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + Display + Debug>(
    _first_list: &[T],
    _second_list: &[T],
) -> Comparison {
    let first_len = _first_list.len();
    let second_len = _second_list.len();

    match (first_len, second_len) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        (first_len, second_len) if (first_len == second_len) => {
            if _first_list[0..] == _second_list[0..] {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
        (first_len, second_len) if (first_len > second_len) => {
            if _first_list
                .windows(second_len)
                .any(|slice| slice == _second_list)
            {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
        (first_len, _) => {
            if _second_list
                .windows(first_len)
                .any(|slice| slice == _first_list)
            {
                return Comparison::Sublist;
            }
            Comparison::Unequal
        }
    }
}
