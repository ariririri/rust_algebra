use num_traits::zero;
use num_traits::one;
use num_traits::Zero;
use std::cmp;
use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Rem};
use std::ops::{AddAssign, MulAssign};
use num_traits::Num;
use num_rational::Rational;



pub trait Coeff: 
  Num 
 +std::clone::Clone
 +AddAssign
{
    fn a(self) ;
}

impl Coeff for i32 {
    fn a(self) {
        print!("{:?}", self);
    }

}

#[derive(Clone)]
pub struct Poly<T: Num> {
    pub f: Vec<T>,
}


impl<T: Num>  Poly<T> {
    pub fn degree(&self) -> usize {
        self.f.len() - 1
    }

    fn reduce(&mut self) -> Poly<T> {
        let degree = self.degree();
        let mut red_deg = 0;
        for i in 0..degree {
            if ! self.f[degree - i].is_zero() { 
                break ;
            }
            red_deg = i+1;
        }

        let g = self.f.drain(0..degree-red_deg+1).collect();
        return Poly { f: g, }
    }
}

impl<T: std::string::ToString +Num> fmt::Debug for Poly<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut _print_poly = "".to_string();
        let dim = self.f.len();
        if self.f[0].is_zero() && self.degree() == 0 {
            _print_poly = _print_poly + "0";
        }
        for n in 0..dim {
            let _dim = dim - 1 - n;
            // 最高次数は+を消す
            // 係数が1の場合は1を非表示
            // 係数が0の場合は消す 
            if self.f[_dim].is_zero()  { continue; }
            if n != 0{
               _print_poly = _print_poly + " + ";
            }
            if ! self.f[dim - 1 -n].is_one() || _dim == 0 {
                _print_poly = _print_poly + &self.f[dim - 1 -n].to_string();
            }
            if n != dim - 1 {
               _print_poly = _print_poly + "x^";
               _print_poly = _print_poly + &_dim.to_string();
             }
        }
        write!(f, "{}", _print_poly)
    }
}

impl<T: Num+std::clone::Clone+AddAssign> AddAssign for Poly<T> {
    fn add_assign(&mut self, other: Self) {
        let max_dim = cmp::max(self.degree(), other.degree());
        for _ in 0..max_dim-self.degree(){ (*self).f.push(zero())};
        for i in 0..other.degree()+1 { (*self).f[i] += other.f[i].clone() }
    }
}

impl<T: Num+std::clone::Clone+AddAssign> Add for &Poly<T> {
    type Output = Poly<T>;
    fn add(self, other: Self) -> Self::Output {
        let max_dim = cmp::max(self.degree(), other.degree());
        let mut out_f: Vec<T> = vec![zero(); max_dim + 1];
        for i in 0..self.degree()+1 { out_f[i] += self.f[i].clone() }
        for i in 0..other.degree()+1 { out_f[i] += other.f[i].clone() }

        Poly {
            f: out_f,
        }.reduce()
    }
} 

impl<T: Num+std::clone::Clone+AddAssign> Add for Poly<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let max_dim = cmp::max(self.degree(), other.degree());
        let mut out_f: Vec<T> = vec![zero(); max_dim + 1];
        for i in 0..self.degree()+1 { out_f[i] += self.f[i].clone() }
        for i in 0..other.degree()+1 { out_f[i] += other.f[i].clone() }

        Self {
            f: out_f,
        }.reduce()
    }
} 


impl<T: Num+std::clone::Clone> Sub for Poly<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let max_dim = cmp::max(self.degree(), other.degree());
        let mut out_f: Vec<T> = vec![zero(); max_dim + 1];
        for i in 0..self.degree()+1 { out_f[i] = out_f[i].clone() + self.f[i].clone() }
        for i in 0..other.degree()+1 { out_f[i] = out_f[i].clone() - other.f[i].clone() }

        Self {
            f: out_f,
        }.reduce()
    }

}

impl<T: Num+std::clone::Clone> Mul for Poly<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let dim = self.degree() + other.degree();
        let mut out_f: Vec<T> = vec![zero(); dim + 1];
        for i in 0..self.degree()+1 { 
            for j in 0..other.degree()+1 {
                out_f[i+j] = out_f[i+j].clone() + self.f[i].clone() * other.f[j].clone()
            }
        }

        Self {
            f: out_f,
        }.reduce()
    }
}

impl<T: Num+std::clone::Clone> MulAssign for Poly<T> {
    // TODO refactor
    fn mul_assign(&mut self, other: Self) {
        let dim = self.degree() + other.degree();
        let mut out_f: Vec<T> = vec![zero(); dim + 1];
        for i in 0..self.degree()+1 { 
            for j in 0..other.degree()+1 {
                out_f[i+j] = out_f[i+j].clone() + self.f[i].clone() * other.f[j].clone()
            }
        }
        (*self).f = out_f;
    }
}

impl<T: Num+std::clone::Clone+std::fmt::Debug> Div for Poly<T> {
    type Output = Self;

    // otherはmonicとする.
    fn  div(self, other: Self) -> Self {
        assert_eq!(other.f[other.degree()], one());
        let mut rem = self.clone();
        let self_d = self.degree() as i32;
        let other_d = other.degree() as i32;
        let deg = self_d - other_d;
        if deg < 0 {return Poly{f: vec![zero(); 1]}};
        let mut divided = Poly{f: vec![zero(); deg as usize+1]};
        for i in 0..deg+1 {
            // あまりの係数
            let target_index = (deg - i + other_d) as usize;
            let coeff = rem.f[target_index].clone();
            // あまりの次数
            let divided_index = (deg - i + 1) as usize;
            let mut _f = vec![zero(); divided_index];
            _f[divided_index-1] = coeff.clone();
            rem = rem - Poly{f: _f} * other.clone();
            divided.f[(deg-i) as usize] = coeff;
        }
        divided

    }
}

impl<T: Num+std::clone::Clone+std::fmt::Debug> Rem for Poly<T> {
    type Output = Self;

    // otherはmonicとする.
    fn  rem(self, other: Self) -> Self {
        assert_eq!(other.f[other.degree()], one());
        let mut rem = self.clone();
        let self_d = self.degree() as i32;
        let other_d = other.degree() as i32;
        let deg = self_d - other_d;
        if deg < 0 {return Poly{f: vec![zero(); 1]}};
        let mut divided = Poly{f: vec![zero(); deg as usize+1]};
        for i in 0..deg+1 {
            // あまりの係数
            let target_index = (deg - i + other_d) as usize;
            let coeff = rem.f[target_index].clone();
            // あまりの次数
            let divided_index = (deg - i + 1) as usize;
            let mut _f = vec![zero(); divided_index];
            _f[divided_index-1] = coeff.clone();
            rem = rem - Poly{f: _f} * other.clone();
            divided.f[(deg-i) as usize] = coeff;
        }
        rem
    }

}

impl<T: Num+std::clone::Clone>  Poly<T> {
    pub fn scalar(&mut self, a: T) -> Poly<T> {
        let a_poly = Poly{f: vec![a]};
        Poly{f: self.f.clone() } * a_poly
    }
    pub fn substitue(&mut self, a: T) -> T {
         let mut _f = self.f.clone();
         _f.reverse();
         _f.iter().cloned().fold(Zero::zero(), |sub, i| sub * a.clone() + i)
    }
}

impl<T: Num+std::clone::Clone+AddAssign>  Poly<T> {
    pub fn power(&self, p: usize) -> Poly<T> {
        let mut out_poly = self.clone();
        for _ in 0..p-1 {out_poly *= self.clone()};
        out_poly
    }

    pub fn comp(&self, other: &Self) -> Self {
        // gのn乗をループで計算
        // n乗ごとにfの定数倍を出す
        let dim = self.degree() * other.degree();
        let mut composed = Poly{f: vec![zero(); dim + 1]};
        let mut other_power = other.clone();
        composed.f[0] = self.f[0].clone();
        for i in 1..self.degree()+1 {
            let coeff = Poly{f: vec![self.f[i].clone()]};
            composed += coeff * other_power.clone();
            other_power = other_power * other.clone();
        };
        composed
    }

    pub fn df(&self) -> Self {
        let dim = self.degree();
        let mut df_f = vec![];
        let mut j = T::zero();
        for i in 1..dim+1 {
            j += one();
            df_f.push(j.clone() * self.f[i].clone())
        }
        Poly{
            f: df_f
        }

    }

}

// rational Polynomialの場合の話
impl Poly<Rational> {
  pub fn gcd(&self, other: &Self) -> Poly<Rational> {
      let mut f = self.clone();
      let mut g = other.clone();
      if self.degree() < other.degree() {
          f = other.clone();
          g = self.clone();
      };
      while g.degree() > 0 || g.f[0] != zero() {
          let _f = f.clone().rem(g.monic());
          f = g;
          g = _f;
      }
      f.monic()
  }

  pub fn monic(&self) -> Poly<Rational> {
      let deg = self.degree();
      let mut out_f = vec![];
      let coeff = self.f[deg];
      for i in 0..deg+1 {
          out_f.push(self.f[i] / coeff)
      };
      Poly{
          f: out_f
      }
  }

  pub fn squarefree(&self) -> Poly<Rational> {
      let df = self.df();
      self.clone() / self.gcd(&df)
  }
}