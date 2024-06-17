use quadratic_residues::*;
mod lib;
mod tests;

fn main () {
    test_get_residues::test_get_residues_default();
    test_get_residues::test_get_residues_non();
    test_get_residues::test_get_residues_all();
}
