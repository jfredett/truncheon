use std::ops::{Add, Mul, Sub};

use crate::hex::coord::cubic::CubicCoord;


#[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct AxialVector {
    u: isize,
    v: isize
}

impl AxialVector {
    pub fn new(u: isize, v: isize) -> Self {
        AxialVector { u, v }
    }

}

#[derive(PartialEq, Clone, Copy, Hash, Eq)]
pub struct Axial {
    q: isize,
    r: isize
}

impl Axial {
    pub fn new(q: isize, r: isize) -> Self {
        Axial { q, r }
    }
}

impl From<Axial> for CubicCoord {
    fn from(value: Axial) -> Self {
        CubicCoord::new(value.q, value.r, -(value.q + value.r))
    }
}

impl From<&Axial> for Axial {
    fn from(value: &Axial) -> Self {
        *value
    }
}


impl std::fmt::Debug for Axial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}, {:?}]", self.q, self.r)
    }
}

// vector addition
impl Add<AxialVector> for Axial {
    type Output = Axial;

    fn add(self, rhs: AxialVector) -> Self::Output {
        Axial::new(
            self.q + rhs.u,
            self.r + rhs.v
        )
    }
}
impl Sub<AxialVector> for AxialVector {
    type Output = AxialVector;

    fn sub(self, rhs: AxialVector) -> Self::Output {
        self + (-1 * rhs)
    }
}

impl Add<AxialVector> for AxialVector {
    type Output = AxialVector;

    fn add(self, rhs: AxialVector) -> Self::Output {
        AxialVector::new(
            self.u + rhs.u,
            self.v + rhs.v
        )
    }
}

// scalar mult
impl Mul<AxialVector> for isize {
    type Output = AxialVector;

    fn mul(self, rhs: AxialVector) -> Self::Output {
        AxialVector::new(
            self * rhs.u,
            self * rhs.v
        )
    }
}
impl Mul<isize> for AxialVector {
    type Output = AxialVector;

    fn mul(self, rhs: isize) -> Self::Output {
        AxialVector::new(
            self.u * rhs,
            self.v * rhs
        )
    }
}

impl std::fmt::Debug for AxialVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{:?}, {:?}>", self.u, self.v)
    }
}

