// tests/integration_test.rs
use quadratic_residues::quadratic_residues;

#[test]
fn test_quadratic_residues() {
    assert_eq!(quadratic_residues(7), vec![1, 2, 4]);
}
