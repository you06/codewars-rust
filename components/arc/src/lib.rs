use std::sync::Arc;

pub fn arc() {
  let foo = Arc::new(vec![1.0, 2.0, 3.0]);
  let a = foo.clone();
  let b = Arc::clone(&foo);
  let c = vec![1.0, 2.0, 3.0];
  // let d = vec![1.0, 2.0, 3.0];
  println!("{:#?}, {:#?}, {}, {}", a, b, a == b, Arc::new(c) == a);
}
