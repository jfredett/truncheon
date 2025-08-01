use std::ops::Add;

use crate::hex::coord::{axial::vector::AxialVector, cubic::CubicCoord};

#[derive(PartialEq, Clone, Copy, Hash, Eq)]
#[cfg_attr(test, derive(derive_quickcheck_arbitrary::Arbitrary))]
pub struct Axial {
    q: isize,
    r: isize
}

impl Axial {
    pub fn new(q: isize, r: isize) -> Self {
        Axial { q, r }
    }

    pub fn q(&self) -> isize { self.q }
    pub fn r(&self) -> isize { self.r }
    pub fn s(&self) -> isize { -self.q - self.r }
}

impl From<Axial> for CubicCoord {
    fn from(value: Axial) -> Self {
        CubicCoord::new(value.q(), value.r(), value.s())
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
            self.q + rhs.u(),
            self.r + rhs.v()
        )
    }
}



#[cfg(test)]
mod test {
    use super::*;

}
