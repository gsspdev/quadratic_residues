// use quadratic_residues::get_residues;
// use crate::get_residues;
use quadratic_residues::{quad_res::Residues, ModuloNum};
fn main() {
   let res_21 = 21.get_residues();
   println!("{:?}", res_21);


   let twentseventy: i32 = 27;
   let res_27true = ModuloNum {
      p: 27, 
      all: true 
   };
   let res_27false = ModuloNum {
      p: 27, 
      all: false
   };
   let res_27tresult = &res_27true.get_residues();
   let res_27fresult = &res_27false.get_residues();
   println!("{:?}", res_27tresult);
   println!("{:?}", res_27fresult);
   // println!("{:?}", get_residues(27).collect());
   // let res_21 = get_residues::default(21);
   // let res_53 = get_residues::default(53);
   // let res_47 = get_residues::default(47);
   // let res_48 = get_residues::default(48);
   // println!("{:?}", res_21);
   // println!("{:?}", res_53);
   // println!("{:?}", res_47[]);
   // println!("{:?}", res_48);
}





