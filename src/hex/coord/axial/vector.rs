use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};


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
        self + (-1 * rhs)
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

impl std::fmt::Debug for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{:?}, {:?}>", self.u, self.v)
    }
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use crate::hex::cardinal::*;

    use super::*;

    mod vector_laws {
        // use super::*;

        // FIXME: Overflow issues
        // #[quickcheck] fn add_commutative(a: Vector, b: Vector) -> bool { a + b == b + a }
        // #[quickcheck] fn mul_commutative(a: Vector, s: isize) -> bool { a * s == s * a }
        // #[quickcheck] fn sub_anticommutative(a: Vector, b: Vector) -> bool { a - b == -1 * (b - a) }
    }

    #[rstest]
    fn hex_movement_to_axial_vector() {
        assert_eq!(Vector::from(N), Vector::new(0, -1));
        assert_eq!(Vector::from(NE), Vector::new(1, -1));
        assert_eq!(Vector::from(NW), Vector::new(-1, 0));
        assert_eq!(Vector::from(S), Vector::new(0, 1));
        assert_eq!(Vector::from(SE), Vector::new(1, 0));
        assert_eq!(Vector::from(SW), Vector::new(-1, 1));
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
            let v = Vector::new(0,1);
            let u = Vector::new(1,0);
            let xp = Vector::new(1,1);

            assert_eq!(u + v, xp);
        }
    }
}
