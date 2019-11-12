use num_traits::zero;
use num_traits::Zero;
use std::cmp;
use std::fmt;
use std::ops::{Add, Sub, Mul};
use num_traits::Num;

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

impl<T: Add+Num+std::clone::Clone> Add for Poly<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let max_dim = cmp::max(self.degree(), other.degree());
        let mut out_f: Vec<T> = vec![zero(); max_dim + 1];
        for i in 0..self.degree()+1 { out_f[i] = out_f[i].clone() + self.f[i].clone() }
        for i in 0..other.degree()+1 { out_f[i] = out_f[i].clone() + other.f[i].clone() }

        Self {
            f: out_f,
        }.reduce()
    }
    

} 

impl<T: Add+Num+std::clone::Clone> Sub for Poly<T> {
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

impl<T: Add+Num+std::clone::Clone> Mul for Poly<T> {
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


impl<T: Add+Num+std::clone::Clone>  Poly<T> {
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
