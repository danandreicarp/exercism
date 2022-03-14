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
    let is_sublist = |first: &[T], second: &[T]| {
        second.is_empty() || first.windows(second.len()).any(|slice| slice == second)
    };

    match (_first_list, _second_list) {
        (first, second) if first == second => Comparison::Equal,
        (first, second) if is_sublist(first, second) => Comparison::Superlist,
        (first, second) if is_sublist(second, first) => Comparison::Sublist,
        _ => Comparison::Unequal,
    }
}
