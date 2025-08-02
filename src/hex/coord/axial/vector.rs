use std::{ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign}, str::FromStr};


#[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
#[cfg_attr(test, derive(derive_quickcheck_arbitrary::Arbitrary))]
pub struct Vector {
    u: isize,
    v: isize
}

impl Vector {
    pub fn new(u: isize, v: isize) -> Self {
        Vector { u, v }
    }


    pub fn u(&self) -> isize { self.u }
    pub fn v(&self) -> isize { self.v }
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
impl Mul<Vector> for isize {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(
            self * rhs.u,
            self * rhs.v
        )
    }
}

impl Mul<isize> for Vector {
    type Output = Vector;

    fn mul(self, rhs: isize) -> Self::Output {
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

    fn neg(self) -> Self::Output { -1 * self }
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

#[derive(Debug, PartialEq, Eq)]
pub struct ParseVectorError;

impl From<std::num::ParseIntError> for ParseVectorError { fn from(_value: std::num::ParseIntError) -> Self { ParseVectorError } }

impl FromStr for Vector {
    type Err = ParseVectorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (u, v) = s
            .strip_prefix('<')
            .and_then(|s| s.strip_suffix('>'))
            .and_then(|s| s.split_once(','))
            .ok_or(ParseVectorError)?;

        Ok(Vector::new(isize::from_str(u)?, isize::from_str(v)?))
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::hex::cardinal::*;

    use super::*;

    #[quickcheck]
    fn display_parse_roundtrip(p: Vector) -> bool {
        let p_s = format!("{}", p);
        let p_parsed = Vector::from_str(&p_s).unwrap();

        p == p_parsed
    }
    #[quickcheck]
    fn debug_parse_roundtrip(p: Vector) -> bool {
        let p_s = format!("{:?}", p);
        let p_parsed = Vector::from_str(&p_s).unwrap();

        p == p_parsed
    }

    mod vector_laws {
        use super::*;

        // FIXME: Overflow issues
        #[quickcheck] fn add_commutative(a_q: i32, a_r: i32, b_q: i32, b_r: i32) -> bool {
            let a = Vector::new(a_q as isize, a_r as isize);
            let b = Vector::new(b_q as isize, b_r as isize);

            a + b == b + a
        }

        #[quickcheck] fn mul_commutative(a_q: i8, a_r: i8, s_in: i16) -> bool {
            let a = Vector::new(a_q as isize, a_r as isize);
            let s = s_in as isize;

            a * s == s * a
        }

        #[quickcheck] fn sub_anticommutative(a_q: i32, a_r: i32, b_q: i32, b_r: i32) -> bool {
            let a = Vector::new(a_q as isize, a_r as isize);
            let b = Vector::new(b_q as isize, b_r as isize);

            a - b == -1 * (b - a)
        }
    }

    #[rstest]
    #[case(N, "<0,-1>")]
    #[case(NE, "<1,-1>")]
    #[case(NW, "<-1,0>")]
    #[case(S, "<0,1>")]
    #[case(SE, "<1,0>")]
    #[case(SW, "<-1,1>")]
    fn hex_movement_to_axial_vector(#[case] direction: Cardinal, #[case] expected: Vector) {
        assert_eq!(Vector::from(direction), expected);
    }

    mod vector_math {
        use super::*;

        #[rstest]
        fn vector_subtraction() {
            let v = Vector::new(0,1);
            let u = Vector::new(1,0);
            let xp = Vector::new(1,-1);

            assert_eq!(u - v, xp);
        }

        #[rstest]
        fn vector_addition() {
            let mut v = Vector::new(0,1);
            let u = Vector::new(1,0);
            let xp = Vector::new(1,1);

            assert_eq!(u + v, xp);

            v += u;

            assert_eq!(v, xp);

            v -= u;

            assert_eq!(u + v, xp);
        }
    }
}
