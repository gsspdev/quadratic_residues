pub struct SplitVec {
    first_half: Vec<i32>,
    last_half: Vec<i32>,
}

impl SplitVec {
    pub fn get_middle(vec: Vec<i32>) -> usize {
        let len = vec.len();
        let mid = len / 2;
        mid
    }

    pub fn get_first_halve(vec: Vec<i32>) -> Vec<i32> {
        let mid: usize = Self::get_middle(vec.clone());
        let first_halve = vec[..mid].to_vec();
        first_halve
    }

    pub fn get_last_halve(vec: Vec<i32>) -> Vec<i32> {
        let mid = Self::get_middle(vec.clone());
        return vec[mid..].to_vec();
    }

    // pub fn halve_vec(vec: Vec<i32>) -> SplitVec {
    //     let or = vec.clone();
    //     let len = vec.len();
    //     let mid = len / 2; // let mid = get_middle(vec);

    //     let first = vec[..mid].to_vec();
    //     let last = vec[mid..].to_vec();

    //     SplitVec {
    //         first_half: first,
    //         last_half: last
    //     }
    // }
}

#[allow(unused)]
#[cfg(test)]
mod split_halves_tests {
    use super::*;
    use crate::split_vector;
    #[test]
    pub fn first_half_test() {
        let vector_to_split: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(SplitVec::get_first_halve(vector_to_split), [1, 2, 3]);
    }
}
