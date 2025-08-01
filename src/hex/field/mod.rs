
use std::{collections::HashMap, marker::ConstParamTy, ops::{Index, IndexMut}};

use crate::hex::coord::axial::point::Axial;

// NOTE: Should this be combined w/ above `Direction`/`FieldOrientation`? I want to support spiral
// coords and such too at some point.
#[derive(ConstParamTy, PartialEq, Eq)]
pub enum Origin {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

// TODO: Maybe this should be "FieldOrientation" or something like? Would be cool to support spiral
// coords, not convinced this is the right type.
#[derive(ConstParamTy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Direction {
    LR_TB,
    RL_TB,
    LR_BT,
    RL_BT
}

#[derive(Debug)]
pub struct Hexfield<const WIDTH: usize, const HEIGHT: usize, const ORIGIN: Origin, const DIRECTION: Direction, T> where T : Clone {
    contents: HashMap<Axial, T>
}

impl<const WIDTH: usize, const HEIGHT: usize, const ORIGIN: Origin, const DIRECTION: Direction, T>
    Hexfield<WIDTH, HEIGHT, ORIGIN, DIRECTION, T>
where T : Clone {
    pub fn new() -> Self {
        Hexfield {
            contents: HashMap::new()
        }
    }

    pub fn insert(&mut self, key: Axial, value: T) {
        self.contents.insert(key, value);
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, const ORIGIN: Origin, const DIRECTION: Direction, T, Idx> IndexMut<Idx>
for Hexfield<WIDTH, HEIGHT, ORIGIN, DIRECTION, T>
where
    T : Clone,
    Idx : Into<Axial>
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        // FIXME: this is probably wrong.
        self.contents.get_mut(&index.into()).unwrap()
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, const ORIGIN: Origin, const DIRECTION: Direction, T, Idx> Index<Idx>
for Hexfield<WIDTH, HEIGHT, ORIGIN, DIRECTION, T>
where
    T : Clone,
    Idx : Into<Axial>
{
    type Output = T;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.contents[&index.into()]
    }
}


#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn insert_and_retrieve() {
        let mut field : Hexfield<10, 10, { Origin::TopLeft }, { Direction::LR_TB }, isize> = Hexfield::new();
        let coord = Axial::new(0,0);
        let coord2 = Axial::new(0,1);

        field.insert(coord, 1);
        field.insert(coord2, 2);

        assert!(field[&coord] == 1);
        assert!(field[&coord2] != 1);
    }

}
