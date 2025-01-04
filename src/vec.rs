use std::ops::{Deref, DerefMut};
use std::hash::{Hash, Hasher};
use crate::params::{Num, GOAL};

#[derive(Clone, Debug, PartialOrd)]
pub struct Vector([Num; GOAL.len()]);

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.0.iter().zip(other.0.iter()).all(|(a, b)| a == b)
    }
}
impl Hash for Vector {

  fn hash<H: Hasher>(&self, state: &mut H) {
      // Implement the hashing logic for Vector
      // For example, if Vector contains a Vec<f64>, you can hash each element
      for element in &self.0 {
          state.write_u64(element.to_bits());
      }
  }
}
impl Vector {
    pub fn constant(n: Num) -> Vector {
        Vector([n; GOAL.len()])
    }

    pub fn from_slice(ns: &[Num]) -> Vector {
        Vector(ns.try_into().expect("slice have same length"))
    }

    #[inline]
    pub fn map(mut self, function: impl Fn(Num) -> Num) -> Vector {
        for x in &mut self.0.iter_mut() {
            *x = function(*x)
        }
        self
    }
}

impl Deref for Vector {
    type Target = [Num; GOAL.len()];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Vector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

macro_rules! impl_op {
    ($trait:ident, $func:ident, $op:tt) => {
        impl std::ops::$trait<&Self> for Vector {
            type Output = Vector;

            fn $func(mut self, rhs: &Self) -> Self::Output {
                for (x, y) in self.iter_mut().zip(rhs.iter()) {
                    *x $op y;
                }
                self
            }
        }
    }
}

macro_rules! impl_unary {
    ($trait:ident, $func:ident, $op:tt) => {
        impl std::ops::$trait for Vector {
            type Output = Vector;

            fn $func(mut self) -> Self::Output {
                for x in self.iter_mut() {
                    *x = $op(*x)
                }
                self
            }
        }
    };
}

impl_op!(Add, add, +=);
impl_op!(Sub, sub, -=);
impl_op!(Mul, mul, *=);
impl_op!(Div, div, /=);
impl_op!(Rem, rem, %=);
// Removed bitwise and shift operations as they cannot be applied to f64
// Removed Not implementation as it cannot be applied to f64
impl_unary!(Neg, neg, (|x| 0.0 - x));
