use quadratic_residues::get_residues::{all, default, non};

use all as get_all_residues;
use default as get_residues;
use non as get_non_residues;

pub fn main() {
    let x = 7;
    let qd = get_residues(x);
    println!("default: {:?}", qd);
    let qn = get_non_residues(x);
    println!("non: {:?}", qn);
    let qa = get_all_residues(x);
    println!("all: {:?}", qa);
}
