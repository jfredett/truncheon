use std::{ops::{Add, AddAssign, Sub, SubAssign}, str::FromStr};

use crate::hex::coord::{axial, cubic::CubicCoord, pixel};

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

    pub fn origin() -> Self {
        Self::new(0,0)
    }

    pub const RT_3 : f64 = 1.7640508076; // sqrt(3);


    // centerpoint
    pub fn to_pointytop_pixel(&self) -> pixel::Point {
        let q : f64 = self.q as f64;
        let half_r : f64 = self.r as f64 / 2.0;
        let x = Self::RT_3 * (q + half_r);
        let y = 3.0 * half_r;

        pixel::Point::new(x,y)
    }

    // TODO: Cartesian Point/Vector?
    // TODO: Matrix class for vectors (simplifies math)

    pub fn to_flattop_pixel(&self) -> pixel::Point {
        let half_q : f64 = self.q as f64 / 2.0;
        let r : f64 = self.r as f64 ;
        let x = 3.0 * half_q;
        let y = Self::RT_3 * (r + half_q);

        pixel::Point::new(x,y)
    }

    pub fn q(&self) -> isize { self.q }
    pub fn r(&self) -> isize { self.r }
    pub fn s(&self) -> isize { -self.q - self.r }

    pub fn neighbors(&self, _range: isize) -> Vec<Point> {
        todo!()
    }
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

impl From<&axial::Vector> for Point {
    fn from(value: &axial::Vector) -> Self {
        Point::origin() + *value
    }
}

impl From<axial::Vector> for Point {
    fn from(value: axial::Vector) -> Self {
        Point::origin() + value
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.q, self.r)
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?},{:?}]", self.q, self.r)
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

impl Sub<axial::Vector> for axial::Point {
    type Output = axial::Point;

    fn sub(self, rhs: axial::Vector) -> Self::Output {
        axial::Point::new(
            self.q() - rhs.u(),
            self.r() - rhs.v()
        )
    }
}

impl Sub<axial::Point> for axial::Point {
    type Output = axial::Vector;

    fn sub(self, rhs: axial::Point) -> Self::Output {
        axial::Vector::new(
            self.q() - rhs.q(),
            self.r() - rhs.r()
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePointError;

impl From<std::num::ParseIntError> for ParsePointError { fn from(_value: std::num::ParseIntError) -> Self { ParsePointError } }

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (q, r) = s
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .and_then(|s| s.split_once(','))
            .ok_or(ParsePointError)?;

        Ok(Point::new(isize::from_str(q)?, isize::from_str(r)?))
    }
}



#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[quickcheck]
    fn display_parse_roundtrip(p: Point) -> bool {
        let p_s = format!("{}", p);
        let p_parsed = Point::from_str(&p_s).unwrap();

        p == p_parsed
    }
    #[quickcheck]
    fn debug_parse_roundtrip(p: Point) -> bool {
        let p_s = format!("{:?}", p);
        let p_parsed = Point::from_str(&p_s).unwrap();

        p == p_parsed
    }

    #[rstest]
    #[case("[0,0]", "<1,1>", "[1,1]")]
    #[case("[1,0]", "<1,1>", "[2,1]")]
    #[case("[1,0]", "<-1,1>", "[0,1]")]
    #[case("[-1,-1]", "<1,1>", "[0,0]")]
    fn add_vector_to_point(#[case] p: Point, #[case] v: axial::Vector, #[case] x: Point) {
        assert_eq!(p + v, x);
    }

    #[rstest]
    #[case("[0,0]", "<1,1>", "[-1,-1]")]
    #[case("[1,0]", "<1,1>", "[0,-1]")]
    #[case("[1,0]", "<-1,1>", "[2,-1]")]
    #[case("[-1,-1]", "<1,1>", "[-2,-2]")]
    fn sub_vector_to_point(#[case] p: Point, #[case] v: axial::Vector, #[case] x: Point) {
        assert_eq!(p - v, x);
    }
}
