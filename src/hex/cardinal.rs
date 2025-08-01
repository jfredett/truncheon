#[cfg(test)]
use derive_quickcheck_arbitrary::Arbitrary;

use crate::hex::coord::axial::AxialVector;


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

const S : Cardinal = Cardinal::S;
const SE : Cardinal = Cardinal::SE;
const SW : Cardinal = Cardinal::SW;
const N : Cardinal = Cardinal::N;
const NE : Cardinal = Cardinal::NE;
const NW : Cardinal = Cardinal::NW;

impl From<Cardinal> for AxialVector {
    fn from(value: Cardinal) -> Self {
        match value {
            Cardinal::N => { AxialVector::new(0,-1) },
            Cardinal::NE => { AxialVector::new(1,-1) },
            Cardinal::NW => { AxialVector::new(-1,0) },
            Cardinal::S => { AxialVector::new(0,1) },
            Cardinal::SE => { AxialVector::new(1,0) },
            Cardinal::SW => { AxialVector::new(-1,1) }
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
