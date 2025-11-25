#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if check_sublist(first_list, second_list){
        Comparison::Sublist
    }else if check_sublist(second_list, first_list){
        Comparison::Superlist
    } else {
        Comparison::Unequal
    }
}

pub fn check_sublist(first: &[i32], second: &[i32]) -> bool {
    if first.is_empty() {
        return true;
    }
    if first.len() > second.len() {
        return false;
    }
    second.windows(first.len()).any(|window| window == first)
}
