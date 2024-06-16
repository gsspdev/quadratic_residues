/// Returns the quadratic residues of an integer
///
/// Examples:
///
/// use quadratic_residue::{ quadratic_residues, quadratic_non_residues, quadratic_residues_all };
///
/// quadratic_residues(7) => [1, 2, 4]
/// quadratic_non_residues(7) => [3, 5, 6]
/// quadratic_residues_all(7) => [1, 4, 2, 2, 4, 1]

use std::collections::HashSet;


/// Returns unique quadratic residues of an integer
fn quadratic_residues(number: i32) -> Vec<i32> {
    let mut quadratic_residues = quadratic_residues_all(number);
    quadratic_residues = get_unique(quadratic_residues);
    quadratic_residues.sort();
    quadratic_residues
}

/// Returns the quadratic non-residues of an integer
fn quadratic_non_residues(number: i32) -> Vec<i32> {
    let quadratic_residues = quadratic_residues_all(number);
    let mut quadratic_non_residues: Vec<i32> = vec![];

    for x in 1..number {
        if !quadratic_residues.contains(&x) {
            quadratic_non_residues.push(x);
        }
    }

    quadratic_non_residues
}

/// Returns the quadratic residues of an integer, including duplicates
fn quadratic_residues_all(number: i32) -> Vec<i32> {
    let mut v = vec_of_smaller_ints(number);

    for x in v.iter_mut() {
        *x = (*x * *x) % number;
    }

    return v;
}

fn vec_of_smaller_ints(n: i32) -> Vec<i32> {
    (1..n).collect()
}

fn get_unique(vec: Vec<i32>) -> Vec<i32> {
    let unique_only: HashSet<_> = vec.clone().drain(..).collect();
    let mut unique_vec: Vec<_> = unique_only.into_iter().collect();
    unique_vec.sort();
    unique_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_of_smaller_ints() {
        assert_eq!(vec_of_smaller_ints(7), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_quadratic_residue() {
        assert_eq!(quadratic_residues_all(7), vec![1, 4, 2, 2, 4, 1]);
    }

    #[test]
    fn test_unique_quadratic_residues() {
        assert_eq!(quadratic_residues(7), vec![1, 2, 4]);
    }

    #[test]
    fn test_quadratic_non_residue() {
        assert_eq!(quadratic_non_residues(7), vec![3, 5, 6]);
    }
}
