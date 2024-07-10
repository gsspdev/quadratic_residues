#[allow(unused)]
pub fn ints_less_than_n(n: i32) -> Vec<i32> {
    (1..n).collect()
}

#[allow(unused_imports)]
#[cfg(test)]
mod utilites_tests {
    use super::*;

    #[test]
    pub fn ints_less_than_n_test() {
        assert_eq!(ints_less_than_n(6), vec![1, 2, 3, 4, 5]);
    }
}
