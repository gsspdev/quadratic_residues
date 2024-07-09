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

pub mod pos_mod {
    pub fn abs_mod(x: i32, r: i32) -> i32 {
        let abs_x = x.abs();
        let abs_r = r.abs();

        let mut result = abs_x % abs_r;
        if result < 0 {
            result += r;
        }
        result
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod utilites_tests {
    use super::*;

    #[test]
    pub fn ints_less_than_n_test() {
        assert_eq!(ints_less_than_n(6), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    pub fn get_unique_test() {
        assert_eq!(get_unique(vec![2, 2, 3, 3, 4, 4, 5, 5, 5, 6, 6, 6, 7, 7, 7, 7, 7]), vec![2, 3, 4, 5, 6, 7]);
    }

    #[test]
    pub fn pos_mod_test() {

    }
}