mod tests;
mod utilities;

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

pub mod get_residues {
    use crate::utilities::get_unique;  // removes duplicates from vector
    use crate::utilities::ints_less_than_n;  // generates vector of ints < n

    /// Gets quadratic residues
    pub fn default(number: i32) -> Vec<i32> {
        let mut default_vector = all(number);
        default_vector = get_unique(default_vector);
        default_vector
    }

    /// Gets non-residues (FAILING TEST)
    pub fn non(number: i32) -> Vec<i32> {
        let non_quadratic_residues = ints_less_than_n(number);
        let residues = non_quadratic_residues.iter().map(|x| x * x).collect();
        residues
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
    // use non as get_non_residues;
    // use all as get_all_residues;

    #[test]
    fn default_test() {
        assert_eq!(get_residues(27), vec![0, 1, 4, 7, 9, 10, 13, 16, 19, 22, 25]);
    }
        // non_residuesk
}