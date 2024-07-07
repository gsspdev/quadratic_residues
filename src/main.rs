use quadratic_residues::get_residues::default as get_residues;

pub fn main() {
    let x = 27;
    let mut q = get_residues(x);
    println!("{:?}", q)
}
