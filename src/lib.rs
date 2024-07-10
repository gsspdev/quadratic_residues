#[allow(unused)]
mod tests;
mod utilities;

pub mod get_residues {
    // use crate::utilities::get_unique;  // removes duplicates from vector
    use crate::utilities::ints_less_than_n; // generates vector of ints < n

    // Although this may have more optimizations,
    // this should be faster than the previous implementation
    // which has been renaimed to slow. There is an n/2 optimization
    // by only computing the first half of the vector.
    // The last half mirrors the first half.
    pub fn optimized_but_failing(number: i32) -> Vec<i32> {
        let n = number.abs();
        let ints_vec = ints_less_than_n(n);
        let midpoint = ints_vec.len() / 2;
        let first_half = ints_vec[..midpoint].to_vec();
        let mut res = first_half.clone();

        res.iter_mut()
            .map(|x| *x = (*x * *x) % number)
            .for_each(drop);
        res.sort();
        res
    }
    /// Gets quadratic residues
    pub fn default(number: i32) -> Vec<i32> {
        let n = number.abs();
        let mut residues = all(n);
        residues.sort();
        residues.dedup();
        residues
    }

    /// Gets non-residues
    pub fn non(number: i32) -> Vec<i32> {
        let n = number.abs();
        let vector: Vec<i32> = ints_less_than_n(n);
        let residues: Vec<i32> = default(n);

        let non_residues: Vec<i32> = vector
            .into_iter()
            .filter(|x| !residues.contains(x))
            .collect();
        non_residues
    }

    /// Gets residues, includes duplicates
    // There is an n/2 optimization here, using default,
    // and the mirrored output of default.
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
        assert_eq!(
            get_residues(27),
            vec![0, 1, 4, 7, 9, 10, 13, 16, 19, 22, 25]
        );
    }
}
