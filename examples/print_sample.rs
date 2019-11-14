extern crate rust_algebra;
use rust_algebra::poly::Poly;
use num_rational::*;

fn main() {
// ライブラリhogeの使用例
  println!("{:?}", rust_algebra::poly::Poly{ f: vec![3.0, 4.1]});

  let mut a = Poly{f: vec![1, 2]};
  let b = Poly{f: vec![1, 2]};
  let c = Poly{f: vec![1, 2]};
  let d = Poly{f: vec![1, 2]};
  let e = Poly{f: vec![1, 2]};
  let di = Poly{f: vec![2, 1]};
  let mut g = &a + &b;
  let h = e - d;
  a += b;
  // a *= c;
  println!("{:?}", g);
  println!("{:?}", h.degree());
  println!("{:?}", g.scalar(3));
  println!("{:?}", g.substitue(3));
  // println!("{:?}", (&a).power(3));
  // println!("{:?}", a.comp(c));
  println!("{:?}", di.clone() % di.clone());
  println!("{:?}", di.clone() / di.clone());
  println!("df {:?}", di.df());
  let rat = Ratio::new(-1, -2);
  let rat_2 = Ratio::new(-10, -2);
  let rat_3 = Ratio::new(-5, -2);
  let rat_one = Ratio::new(1, 1);
  let rat_poly = Poly{f: vec![rat; 3]};
  println!("start rational {:?}", rat_poly.monic());
  let rat_zero = Ratio::new(0, 1);
  let rat_poly_2 = Poly{f: vec![rat, rat_2, rat_3]};
  let rat_poly_3 = Poly{f: vec![- rat_one, rat_zero, rat_zero, rat_one]};
  let rat_poly_4 = Poly{f: vec![rat_zero, rat_one ]};
  println!("{:?}", rat_poly.monic());
  println!("{:?}", rat_poly.gcd(&rat_poly_2));
  println!("{:?}", rat_poly_3.gcd(&rat_poly));
  println!("{:?}", rat_poly_4.squarefree());
}