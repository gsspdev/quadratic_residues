use quadratic_residues::get_residues::{
    get_all_residues, get_non_residues, get_residues, get_residues_optimized_but_failing,
};

// use default as get_residues
// use all as get_all_residues;
// use non as get_non_residues;
// use optimized_but_failing as get_residues_optimized_but_failing;

pub fn main() {
    let x = 2141;

    let qr = get_residues(x);
    println!("default: {:?}", qr);

    let qn = get_non_residues(x);
    println!("non: {:?}", qn);

    let qa = get_all_residues(x);
    println!("all: {:?}", qa);

    let qs = get_residues_optimized_but_failing(x);
    println!("optimized but possibly failing: {:?}", qs);
}
