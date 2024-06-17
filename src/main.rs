use quadratic_residues::*;
use quadratic
mod lib::*;
mod tests::test_get_residues;

fn main () {
    test_get_residues::test_get_residues_default();
    test_get_residues::test_get_residues_non();
    test_get_residues::test_get_residues_all();
}
