pub trait Splittable<T> {}

pub struct SplitVec {
    first_half: Vec<i32>,
    last_half: Vec<i32>,
}

impl SplitVec {
    fn new<T>(first_half: Vec<T>, last_half: Vec<T>) -> Self {
        Self { first_half, last_half }
    }
}

impl Splittable for Vec<i32> {
    pub fn new() -> SplitVec {
        SplitVec {
            first_half: first,
            last_half: last
        }
    }

    fn get_vec_middle(&self) -> usize { self.len() / 2 }
    fn split_vec(&self, vec: Vec<i32>) -> SplitVec {
        let mid = Self::get_vec_middle(&self);
        SplitVec::new(vec[..mid].to_vec(), vec[mid..].to_vec())
    }

    fn get_first_halve(split_vecs :SplitVec) -> Vec<i32> { split_vecs.first_half }
    fn get_last_halve(split_vecs: SplitVec) -> Vec<i32> { split_vecs.last_half }
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
