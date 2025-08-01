#[cfg(test)]
use derive_quickcheck_arbitrary::Arbitrary;

use crate::hex::coord::axial;


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum Cardinal {
    N,
    NE,
    NW,
    S,
    SE,
    SW
}

pub const S : Cardinal = Cardinal::S;
pub const SE : Cardinal = Cardinal::SE;
pub const SW : Cardinal = Cardinal::SW;
pub const N : Cardinal = Cardinal::N;
pub const NE : Cardinal = Cardinal::NE;
pub const NW : Cardinal = Cardinal::NW;

impl From<Cardinal> for axial::Vector {
    fn from(value: Cardinal) -> Self {
        match value {
            Cardinal::N => { axial::Vector::new(0,-1) },
            Cardinal::NE => { axial::Vector::new(1,-1) },
            Cardinal::NW => { axial::Vector::new(-1,0) },
            Cardinal::S => { axial::Vector::new(0,1) },
            Cardinal::SE => { axial::Vector::new(1,0) },
            Cardinal::SW => { axial::Vector::new(-1,1) }
        }
    }
}

impl Cardinal {
    pub fn clockwise(&self) -> Self {
        match self {
            Cardinal::N => { NE },
            Cardinal::NE => { SE },
            Cardinal::SE => { S },
            Cardinal::S => { SW },
            Cardinal::SW => { NW },
            Cardinal::NW => { N },
        }
    }

    pub fn counterclockwise(&self) -> Self {
        match self {
            Cardinal::N => { NW },
            Cardinal::NW => { SW },
            Cardinal::SW => { S },
            Cardinal::S => { SE },
            Cardinal::SE => { NE },
            Cardinal::NE => { N },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[quickcheck]
    fn rotation_inverse(h: Cardinal) -> bool {
        h.clockwise().counterclockwise() == h && h.counterclockwise().clockwise() == h
    }
}
