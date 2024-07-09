use std::collections::HashSet;

pub fn ints_less_than_n(n: i32) -> Vec<i32> {
    (1..n).collect()
}

pub fn get_unique(vec: Vec<i32>) -> Vec<i32> {
    let unique_only: HashSet<_> = vec.clone().drain(..).collect();
    let mut unique_vec: Vec<_> = unique_only.into_iter().collect();
    unique_vec.sort();
    unique_vec
}
