use std::ops::{Add, Sub, Mul};


#[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct AxialVector {
    u: isize,
    v: isize
}

impl AxialVector {
    pub fn new(u: isize, v: isize) -> Self {
        AxialVector { u, v }
    }


    pub fn u(&self) -> isize { self.u }
    pub fn v(&self) -> isize { self.v }
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
