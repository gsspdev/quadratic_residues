use std::collections::HashSet;
use itertools::Itertools;

pub fn ints_less_than_n(n: i32) -> Vec<i32> {
    (1..n).collect()
}

pub fn get_unique(vec: Vec<i32>) -> Vec<i32> {
    let unique_only: HashSet<_> = vec.clone().drain(..).collect();
    let unique_vec: Vec<_> = unique_only.into_iter().sorted().dedup().collect();
    unique_vec
}

pub fn abs_rem(div: i32, dvsr: i32) -> i32 {
    let rem = div.abs() % dvsr.abs();
    rem
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
    pub fn dedup_test() {
        assert_eq!(vec![1, 1, 2, 3, 2, 3, 4, 5].into_iter().sorted().dedup().collect_vec() ,vec![1, 2, 3, 4, 5])
    }

    mod abs_rem_tests {
        use super::abs_rem;
        #[test]
        pub fn neg_div_test() {
            assert_eq!(abs_rem(-7, 5), 2);
        }

        #[test]
        pub fn neg_dvsr_test() {
            assert_eq!(abs_rem(7, -5), 2);
        }
        
        #[test]
        pub fn neg_both_test() {
            assert_eq!(abs_rem(-7, -5), 2);
        }
    }
}
