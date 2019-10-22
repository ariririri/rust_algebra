extern crate rust_algebra;
use rust_algebra::poly::Poly;

fn main() {
// ライブラリhogeの使用例
  println!("{:?}", rust_algebra::poly::Poly{ f: vec![3.0, 4.1]});

  let mut a = Poly{f: vec![1, 2]};
  let mut b = Poly{f: vec![1, 2]};
  let c = Poly{f: vec![1, 2]};
  let d = Poly{f: vec![1, 2]};
  let e = Poly{f: vec![1, 2]};
  let mut g = a + b;
  let h = b - d;
  println!("{:?}", g);
  println!("{:?}", h.degree());
  println!("{:?}", g.scalar(3));
  println!("{:?}", g.substitue(3));
}