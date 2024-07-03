




trait MyFunction {
  fn execute(&self);
}

impl MyFunction for i32 {
  fn execute(&self) {
      println!("Function called with int: {}", self);
  }
}

impl MyFunction for (i32, bool) {
  fn execute(&self) {
      let (int_val, bool_val) = *self;
      println!("Function called with int: {} and bool: {}", int_val, bool_val);
  }
}

fn main() {
  let a = 5;
  a.execute(); // Calls the i32 implementation

  let b = (10, true);
  b.execute(); // Calls the (i32, bool) implementation
}
