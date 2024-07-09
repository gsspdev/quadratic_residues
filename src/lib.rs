#[allow(unused)]
mod tests;
mod utilities;

pub mod get_residues {
    use crate::utilities::get_unique;  // removes duplicates from vector
    use crate::utilities::ints_less_than_n;  // generates vector of ints < n

    /// Gets quadratic residues
    pub fn default(number: i32) -> Vec<i32> {
        let n = number.abs();
        let mut default_vector = all(n);
        default_vector = get_unique(default_vector);
        default_vector
    }

    /// Gets non-residues (FAILING TEST)
    pub fn non(number: i32) -> Vec<i32> {
        let n = number.abs();
        let vector: Vec<i32> = ints_less_than_n(n) ;
        let residues: Vec<i32> = default(n);
        
        let non_residues: Vec<i32> =  vector.into_iter()
                                            .filter(|x| !residues.contains(x))
                                            .collect();
        non_residues
    }

    /// Gets residues, includes duplicates
    pub fn all(number: i32) -> Vec<i32> {
        let mut v = ints_less_than_n(number);
        for x in v.iter_mut() {
            *x = (*x * *x) % number;
        }
        return v;
    }

}

#[allow(unused)]
#[cfg(test)]
mod testing {
    use super::get_residues::{all, default, non};
    use default as get_residues;

    #[test]
    fn default_test() {
        assert_eq!(get_residues(27), vec![0, 1, 4, 7, 9, 10, 13, 16, 19, 22, 25]);
    }

}