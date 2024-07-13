#[cfg(test)]
pub mod get_residues_tests {
    use crate::get_residues;

    #[test]
    pub fn get_residues_default_test() {
        assert_eq!(get_residues::default(7), vec![1, 2, 4]);
    }

    #[test]
    pub fn get_residues_optimized_but_failing_test() {
        assert_eq!(get_residues::optimized_but_failing(7), vec![1, 2, 4]);
    }

    #[test]
    pub fn get_residues_non_test() {
        assert_eq!(get_residues::non(7), vec![3, 5, 6], "Failed to get non-residues");
    }

    #[test]
    pub fn get_residues_all_test() {
        assert_eq!(get_residues::all(7), vec![1, 4, 2, 2, 4, 1]);
    }
}
