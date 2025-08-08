use std::{ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign}, str::FromStr};

use crate::hex::coord::pixel;


#[derive(PartialEq, Clone, Copy, Default)]
#[cfg_attr(test, derive(derive_quickcheck_arbitrary::Arbitrary))]
pub struct Vector {
    u: f64,
    v: f64
}

impl Vector {
    pub fn new(u: f64, v: f64) -> Self {
        Self { u, v }
    }

    pub fn u(&self) -> f64 {
        self.u
    }

    pub fn v(&self) -> f64 {
        self.v
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        self + -rhs
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector::new(
            self.u + rhs.u,
            self.v + rhs.v
        )
    }
}

// scalar mult
impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(
            self * rhs.u,
            self * rhs.v
        )
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector::new(
            self.u * rhs,
            self.v * rhs
        )
    }
}

impl AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.u += rhs.u;
        self.v += rhs.v;
    }
}

impl SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, rhs: Vector) {
        self.u -= rhs.u;
        self.v -= rhs.v;
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output { -1.0 * self }
}

impl std::fmt::Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{:?},{:?}>", self.u, self.v)
    }
}
impl std::fmt::Debug for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{:?},{:?}>", self.u, self.v)
    }
}

impl From<&pixel::Point> for Vector {
    fn from(value: &pixel::Point) -> Self {
        Vector::from(*value)
    }
}
impl From<pixel::Point> for Vector {
    fn from(value: pixel::Point) -> Self {
        value - pixel::Point::origin()
    }
}
