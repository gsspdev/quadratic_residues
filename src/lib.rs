pub mod pos_mod {
    pub fn abs_mod(x: i32, r: i32) -> i32 {
        let abs_x = x.abs();
        let abs_r = r.abs();

        let mut result = abs_x % abs_r;
        if result < 0 {
            result += abs_r;
        }
        result
    }
}

pub struct ModuloNum {
    pub p: i32,
    pub all: bool
}

pub mod quad_res {
    use crate::ModuloNum;

    pub trait Residues {
        fn get_residues(&self) -> Vec<i32>;
    }

    impl Residues for i32 {
        fn get_residues(&self) -> Vec<i32> {
            let p = *self;
            let mut v: Vec<i32> = (1..p).collect();
            for x in v.iter_mut() {
                *x = (*x * *x) % p;
            }
            v
        }
    }

    impl Residues for ModuloNum {
        fn get_residues(&self) -> Vec<i32> {
            get_residues_all(self.p, self.all)
        }
    }

    impl Residues for (i32, bool) {
        fn get_residues(&self) -> Vec<i32> {
            let (p, all) = *self;
            get_residues_all(p, all)
        }
    }
    
    pub fn get_residues_all(p: i32, all: bool) -> Vec<i32> {
        let mut v: Vec<i32> = (1..p).collect();
        for x in v.iter_mut() {
            *x = (*x * *x) % p;
        }
        v.sort_unstable();
        if !all {
            v.dedup();
        }
        v
    }
    
}
