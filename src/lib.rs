mod tests;

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
    use std::collections::HashSet;

    /// Returns unique quadratic residues of an integer
    pub fn default(number: i32) -> Vec<i32> {
        let mut default_vector = all(number);
        default_vector = get_unique(default_vector);
        default_vector
    }

    /// Returns the quadratic non-residues of an integer
    pub fn non(number: i32) -> Vec<i32> {
        let quadratic_residues = ints_less_than_n(number);
        let mut non_residues: Vec<i32> = vec![];

        for x in 1..number {
            if !quadratic_residues.contains(&x) {
                non_residues.push(x);
            }
        }

        non_residues
    }

    /// Returns the quadratic residues of an integer, including duplicates pub fn quadratic_residues_all(number: i32) -> Vec<i32> {
    pub fn all(number: i32) -> Vec<i32> {
        let mut v = ints_less_than_n(number);
        for x in v.iter_mut() {
            *x = (*x * *x) % number;
        }
        return v;
    }

    fn ints_less_than_n(n: i32) -> Vec<i32> {
        (1..n).collect()
    }

    fn get_unique(vec: Vec<i32>) -> Vec<i32> {
        let unique_only: HashSet<_> = vec.clone().drain(..).collect();
        let mut unique_vec: Vec<_> = unique_only.into_iter().collect();
        unique_vec.sort();
        unique_vec
    }
}
