## Quadratic Residues -- A Brief Overview

"In number theory, an integer q is called a quadratic residue modulo n if it is congruent to a perfect square modulo n; i.e., if there exists an integer x such that: $$x^2 \equiv q \pmod{n}$$
Otherwise, q is called a quadratic nonresidue modulo n.

Originally an abstract mathematical concept from the branch of number theory known as modular arithmetic, quadratic residues are now used in applications ranging from acoustical engineering to cryptography and the factoring of large numbers."

Source: <https://en.wikipedia.org/wiki/Quadratic_residue>

For a quick overview of quadratic residues I recommend [this](https://www.youtube.com/watch?v=aBn7BaRxu2g 'Number Theory | Quadratic Residues: Definition and Examples') video from [Michael Penn](https://www.youtube.com/@MichaelPennMath '@MichaelPennMath').

## Usage

### Add the Crate to Your Project

```bash
cargo add quadratic_residues
```

### or manually specify in Cargo.toml

```toml
[dependencies]
quadratic_residues = "0.1.4"
```

## Examples

```rust
get_residues(7) => [1, 2, 4] // returns the quadratic residues of 7
get_non_residues(7) => [3, 5, 6] // returns the quadratic non-residues of 7
get_all_residues(7) => [1, 4, 2, 2, 4, 1] // returns the quadratic residues of 7 including duplicates
```

## TODO

    - add quadratic_residues_all() to include redundant integers for symmetry lovers [x]
    - add T.quad_res(), T.non_res(), & T.all_res() [in progress]
    - Impl Trait for signed and unsigned integers [in progress]
    - improve test robustness [in progress]

## 1.0.0 release roadmap

    - add n/2 optimization to double speed []
        - sidestep de_dep-ing filter as micro-optimization [] 
        - modify quadratic_residues_all behavior to return Vec<> from quadratic_residues() with it's reverse []
    - account for rust '%' operator's handling of negative values []
    - change naming conventions []
    - full Impl coverage of std lib integer types []

## More to come

## TODO

    - add quadratic_residues_all() to include redundant integers for symmetry lovers [x]
    - add T.quad_res(), T.non_res(), & T.all_res() [in progress]
    - Impl Trait for signed and unsigned integers [in progress]
    - improve test robustness [in progress]

## 1.0.0 release roadmap

    - add n/2 optimization to double speed []
        - sidestep de_dep-ing filter as micro-optimization [] 
        - modify quadratic_residues_all behavior to return Vec<> from quadratic_residues() with it's reverse []
    - account for rust '%' operator's handling of negative values []
    - change naming conventions []
    - full Impl coverage of std lib integer types []
