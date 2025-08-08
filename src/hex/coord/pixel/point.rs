use std::{ops::{Add, AddAssign, Sub, SubAssign}, str::FromStr};

use ratatui::{style::Color, widgets::canvas::Line};

use crate::hex::coord::pixel;


#[derive(PartialEq, Clone, Copy, Default)]
#[cfg_attr(test, derive(derive_quickcheck_arbitrary::Arbitrary))]
pub struct Point {
    x: f64,
    y: f64
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[|{:?},{:?}|]", self.x, self.y)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePointError;

impl From<std::num::ParseFloatError> for ParsePointError { fn from(_value: std::num::ParseFloatError) -> Self { ParsePointError } }

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (u, v) = s
            .strip_prefix("[|")
            .and_then(|s| s.strip_suffix("|]"))
            .and_then(|s| s.split_once(','))
            .ok_or(ParsePointError)?;

        Ok(Point::new(f64::from_str(u)?, f64::from_str(v)?))
    }
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn origin() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn line(p1: Point, p2: Point) -> Line {
        Line {
            x1: p1.x, y1: p1.y,
            x2: p2.x, y2: p2.y,
            color: Color::Red
        }
    }
}

impl From<&Point> for Point {
    fn from(value: &Point) -> Self {
        *value
    }
}

impl From<&pixel::Vector> for Point {
    fn from(value: &pixel::Vector) -> Self {
        Point::origin() + *value
    }
}

impl From<pixel::Vector> for Point {
    fn from(value: pixel::Vector) -> Self {
        Point::origin() + value
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


// vector addition
impl Add<pixel::Vector> for Point {
    type Output = Point;

    fn add(self, rhs: pixel::Vector) -> Self::Output {
        Point::new(
            self.x + rhs.u(),
            self.y + rhs.v()
        )
    }
}

impl AddAssign<pixel::Vector> for Point {
    fn add_assign(&mut self, rhs: pixel::Vector) {
        self.x += rhs.u();
        self.y += rhs.v();
    }
}

impl SubAssign<pixel::Vector> for Point {
    fn sub_assign(&mut self, rhs: pixel::Vector) {
        self.x -= rhs.u();
        self.y -= rhs.v();
    }
}

impl Sub<pixel::Vector> for pixel::Point {
    type Output = pixel::Point;

    fn sub(self, rhs: pixel::Vector) -> Self::Output {
        pixel::Point::new(
            self.x() - rhs.u(),
            self.y() - rhs.v()
        )
    }
}

impl Sub<pixel::Point> for pixel::Point {
    type Output = pixel::Vector;

    fn sub(self, rhs: pixel::Point) -> Self::Output {
        pixel::Vector::new(
            self.x() - rhs.x(),
            self.y() - rhs.y()
        )
    }
}


#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[quickcheck]
    fn display_parse_roundtrip(p: Point) -> bool {
        if p.x.is_nan() || p.y.is_nan() { return true; }

        let p_s = format!("{}", p);
        let p_parsed = Point::from_str(&p_s).unwrap();

        p == p_parsed
    }
    #[quickcheck]
    fn debug_parse_roundtrip(p: Point) -> bool {
        if p.x.is_nan() || p.y.is_nan() { return true; }

        let p_s = format!("{:?}", p);
        let p_parsed = Point::from_str(&p_s).unwrap();

        p == p_parsed
    }

    // #[rstest]
    // #[case("[0,0]", "<1,1>", "[1,1]")]
    // #[case("[1,0]", "<1,1>", "[2,1]")]
    // #[case("[1,0]", "<-1,1>", "[0,1]")]
    // #[case("[-1,-1]", "<1,1>", "[0,0]")]
    // fn add_vector_to_point(#[case] p: Point, #[case] v: pixel::Vector, #[case] x: Point) {
    //     assert_eq!(p + v, x);
    // }

    // #[rstest]
    // #[case("[0,0]", "<1,1>", "[-1,-1]")]
    // #[case("[1,0]", "<1,1>", "[0,-1]")]
    // #[case("[1,0]", "<-1,1>", "[2,-1]")]
    // #[case("[-1,-1]", "<1,1>", "[-2,-2]")]
    // fn sub_vector_to_point(#[case] p: Point, #[case] v: pixel::Vector, #[case] x: Point) {
    //     assert_eq!(p - v, x);
    // }
}
