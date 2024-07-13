#[cfg(test)]
use crate::get_residues;
#[test]
pub mod get_residues_tests {
    use crate::get_residues;

    #[cfg(test)]
    #[test]
    pub fn test_get_residues_default() {
        assert_eq!(get_residues::default(47), vec![1, 2, 4]);
    }

    #[test]
    pub fn test_get_residues_non() {
        assert_eq!(get_residues::non(47), vec![3, 5, 6]);
    }

    #[test]
    pub fn test_get_residues_all() {
        assert_eq!(get_residues::all(47), vec![1, 4, 2, 2, 4, 1]);
    }
}
