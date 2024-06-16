# Rust Crate for Calculating Quadratic Residues of an Integer

## Description

### From Wikipedia: [Quadratic Residue](https://en.wikipedia.org/wiki/Quadratic_residue)

"In number theory, an integer q is called a quadratic residue modulo n if it is congruent to a perfect square modulo n; i.e., if there exists an integer x such that:
\[ x^2 \equiv q \pmod{n} \]
Otherwise, q is called a quadratic nonresidue modulo n.

Originally an abstract mathematical concept from the branch of number theory known as modular arithmetic, quadratic residues are now used in applications ranging from acoustical engineering to cryptography and the factoring of large numbers."

## Usage

### Add the Crate to Your Project

Using cargo:
```bash
cargo add quadratic_residues
```
### or add it manually to your Cargo.toml file:

```toml
[dependencies]
quadratic_residues = "0.1.2"
```

### To use the crate in your project:
```rust
use quadratic_residues::{ quadratic_residues, quadratic_non_residues, quadratic_residues_all };
```

## Examples:

```rust
quadratic_residues(7) => [1, 2, 4] // returns the quadratic residues of 7
quadratic_non_residues(7) => [3, 5, 6] // returns the quadratic non-residues of 7
quadratic_residues_all(7) => [1, 4, 2, 2, 4, 1] // returns the quadratic residues of 7 including duplicates
```
