use std::ops::{Add, AddAssign, SubAssign};

use crate::hex::coord::{axial, cubic::CubicCoord};

#[derive(PartialEq, Clone, Copy, Hash, Eq)]
#[cfg_attr(test, derive(derive_quickcheck_arbitrary::Arbitrary))]
pub struct Point {
    q: isize,
    r: isize
}

impl Point {
    pub fn new(q: isize, r: isize) -> Self {
        Point { q, r }
    }

    pub fn q(&self) -> isize { self.q }
    pub fn r(&self) -> isize { self.r }
    pub fn s(&self) -> isize { -self.q - self.r }
}

impl From<Point> for CubicCoord {
    fn from(value: Point) -> Self {
        CubicCoord::new(value.q(), value.r(), value.s())
    }
}

impl From<&Point> for Point {
    fn from(value: &Point) -> Self {
        *value
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}, {:?}]", self.q, self.r)
    }
}

// vector addition
impl Add<axial::Vector> for Point {
    type Output = Point;

    fn add(self, rhs: axial::Vector) -> Self::Output {
        Point::new(
            self.q + rhs.u(),
            self.r + rhs.v()
        )
    }
}

impl AddAssign<axial::Vector> for Point {
    fn add_assign(&mut self, rhs: axial::Vector) {
        self.q += rhs.u();
        self.r += rhs.v();
    }
}

impl SubAssign<axial::Vector> for Point {
    fn sub_assign(&mut self, rhs: axial::Vector) {
        self.q -= rhs.u();
        self.r -= rhs.v();
    }
}



#[cfg(test)]
mod test {
    // use super::*;


}
