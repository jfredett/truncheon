#![allow(dead_code)]


/*
* Loose sources/notes


// for eventually calculating distance-to-horizon/number of revealed tiles

http://www.totally-cuckoo.com/distance_visible_to_the_horizon.htm

Height (ft)      Height (m)      Distance (miles)      Distance (km)
5                0.98            3.24                  5.21
10               3.05            4.18                  6.76
25               7.62            6.61                  10.62
50               15.24           9.35                  15.13
100              30.48           12.23                 19.63
200              60.96           18.72                 30.09
400              121.92          26.46                 42.65
1000             304.80          32.41                 52.14

The source of all hex magic:
https://www.redblobgames.com/grids/hexagons/

*/

use std::{collections::HashMap, marker::ConstParamTy, ops::{Add, Index, IndexMut, Mul}};

#[cfg(test)]
use derive_quickcheck_arbitrary::Arbitrary;


#[derive(PartialEq, Clone, Copy, Hash, Eq)]
pub struct AxialCoord {
    q: isize,
    r: isize
}

impl AxialCoord {
    pub fn new(q: isize, r: isize) -> Self {
        AxialCoord { q, r }
    }
}

impl From<AxialCoord> for CubicCoord {
    fn from(value: AxialCoord) -> Self {
        CubicCoord::new(value.q, value.r, -(value.q + value.r))
    }
}

impl From<&AxialCoord> for AxialCoord {
    fn from(value: &AxialCoord) -> Self {
        *value
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub struct CubicCoord {
    q: isize,
    r: isize,
    s: isize
}

impl CubicCoord {
    pub fn new(q: isize, r: isize, s: isize) -> Self {
        assert!(q + r + s == 0);
        CubicCoord { q, r, s }
    }
}

/// Maps a coordinate to a index counting from the origin provided
struct IndexCoord<const ORIGIN: Origin, const DIRECTION: Direction> {

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

// NOTE: Should this be combined w/ above `Direction`/`FieldOrientation`? I want to support spiral
// coords and such too at some point.
#[derive(ConstParamTy, PartialEq, Eq)]
pub enum Origin {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight
}

#[derive(ConstParamTy, PartialEq, Eq)]
pub enum OffsetType {
    OddQ,
    OddR,
    EvenQ,
    EvenR
}

pub struct OffsetCoord<const TYPE: OffsetType> {
    x: isize,
    y: isize
}

///
/// ```
/// assert!(true);
///
/// ```
#[derive(Debug)]
pub struct Hexfield<const WIDTH: usize, const HEIGHT: usize, const ORIGIN: Origin, const DIRECTION: Direction, T> where T : Clone {
    contents: HashMap<AxialCoord, T>
}

impl<const WIDTH: usize, const HEIGHT: usize, const ORIGIN: Origin, const DIRECTION: Direction, T>
    Hexfield<WIDTH, HEIGHT, ORIGIN, DIRECTION, T>
where T : Clone {

    pub fn new() -> Self {
        Hexfield {
            contents: HashMap::new()
        }
    }

    pub fn insert(&mut self, key: AxialCoord, value: T) {
        self.contents.insert(key, value);
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, const ORIGIN: Origin, const DIRECTION: Direction, T, Idx> IndexMut<Idx>
for Hexfield<WIDTH, HEIGHT, ORIGIN, DIRECTION, T>
where
    T : Clone,
    Idx : Into<AxialCoord>
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
    Idx : Into<AxialCoord>
{
    type Output = T;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.contents[&index.into()]
    }
}


// TODO:
// Something like a `World` struct which contains many `fields`, a `field` is a Hexfield, a
// Depthcrawl, a pointcrawl, etc. Fields contain `tiles` (`hex`es in the case of hexfield,
// locations for depth or pointcrawl), and `tiles` have some set of `connections`; for `hexes`
// there are 6 natural connections (to other hexes), but they might also connect to another field;
// or have one of their natural connections replaced with a connection to another field.
//
// On the `world` struct, there are some set of `cursor`s, which represent players, parties,
// enemies, or other mobile items.ll




#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[cfg_attr(test, derive(Arbitrary))]
pub enum HexMovement {
    N,
    NE,
    NW,
    S,
    SE,
    SW
}

const S : HexMovement = HexMovement::S;
const SE : HexMovement = HexMovement::SE;
const SW : HexMovement = HexMovement::SW;
const N : HexMovement = HexMovement::N;
const NE : HexMovement = HexMovement::NE;
const NW : HexMovement = HexMovement::NW;

impl From<HexMovement> for AxialVector {
    fn from(value: HexMovement) -> Self {
        match value {
            HexMovement::N => { AxialVector::new(0,-1) },
            HexMovement::NE => { AxialVector::new(1,-1) },
            HexMovement::NW => { AxialVector::new(-1,0) },
            HexMovement::S => { AxialVector::new(0,1) },
            HexMovement::SE => { AxialVector::new(1,0) },
            HexMovement::SW => { AxialVector::new(-1,1) }
        }
    }
}

impl HexMovement {
    pub fn clockwise(&self) -> Self {
        match self {
            HexMovement::N => { NE },
            HexMovement::NE => { SE },
            HexMovement::SE => { S },
            HexMovement::S => { SW },
            HexMovement::SW => { NW },
            HexMovement::NW => { N },
        }
    }

    pub fn counterclockwise(&self) -> Self {
        match self {
            HexMovement::N => { NW },
            HexMovement::NW => { SW },
            HexMovement::SW => { S },
            HexMovement::S => { SE },
            HexMovement::SE => { NE },
            HexMovement::NE => { N },
        }
    }



}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
pub struct AxialVector {
    u: isize,
    v: isize
}

impl AxialVector {
    pub fn new(u: isize, v: isize) -> Self {
        AxialVector { u, v }
    }

}




// vector addition
impl Add<AxialVector> for AxialCoord {
    type Output = AxialCoord;

    fn add(self, rhs: AxialVector) -> Self::Output {
        AxialCoord::new(
            self.q + rhs.u,
            self.r + rhs.v
        )
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

impl std::fmt::Debug for AxialCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}, {:?}]", self.q, self.r)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::quickcheck;
    use rstest::rstest;

    #[quickcheck]
    fn rotation_inverse(h: HexMovement) -> bool {
        h.clockwise().counterclockwise() == h && h.counterclockwise().clockwise() == h
    }

    #[rstest]
    #[tracing_test::traced_test]
    fn it_works() {
        let mut field : Hexfield<10, 10, { Origin::TopLeft }, { Direction::LR_TB }, isize> = Hexfield::new();
        let coord = AxialCoord::new(0,0);
        let coord2 = AxialCoord::new(0,1);

        field.insert(coord, 1);
        field.insert(coord2, 2);

        assert!(field[&coord] == 1);
        assert!(field[&coord2] != 1);
    }

    mod vector_math {
        use super::*;

        #[rstest]
        fn vector_addition() {
            let v = AxialVector::new(0,1);
            let u = AxialVector::new(1,0);
            let xp = AxialVector::new(1,1);

            assert_eq!(u + v, xp);
        }

    }

    #[rstest]
    fn hex_movement_to_axial_vector() {
        assert_eq!(AxialVector::from(HexMovement::N), AxialVector::new(0, -1));
        assert_eq!(AxialVector::from(HexMovement::NE), AxialVector::new(1, -1));
        assert_eq!(AxialVector::from(HexMovement::NW), AxialVector::new(-1, 0));
        assert_eq!(AxialVector::from(HexMovement::S), AxialVector::new(0, 1));
        assert_eq!(AxialVector::from(HexMovement::SE), AxialVector::new(1, 0));
        assert_eq!(AxialVector::from(HexMovement::SW), AxialVector::new(-1, 1));
    }
}

/*

The goal of this thing is to reduce as much friction in the crawling process as possible.

Ideally the result is a CLI which talks to Foundry (or a WASM module that I load into Foundry)
which eventually generates a log of each step of the process, tracks time and date, allows
recording of events associated with that, etc.

A simple Ratatui UI which I punch the intended direction into, it should generate the various DCs according
to skill type based on the terrain and it's contents / tags etc.

The system should let me tag the resulting location, maybe record myself describing it, transcribe it, save it, etc.

I don't hate the raw data running to grist, but would prefer it eventually in foundry or in my own DB.

The thing should generate a couple maps -- the real map (where the players actually are), the implied map (where the players think they are), and ideally it should allow for a multi-planar system.

Coordinates should be endowed with an extra coordinate, "Plane" which cooresponds to:

1. Ground level, Earth
2. Sky, Earth
3. Underground, Earth
4. Ynn
5. The Stygian Library
6. etc.

Each plane may or may not be a hexcrawl, so in some planes I might switch to other methods of tracking, this tool is for that.

It should also, ideally, track depthcrawls, pointcrawls, etc. Something like (not to scale):

-------------------------------------------------
| NOTES  |  REAL MAP |  PLAYER MAP | TRAVEL LOG |
| NOTES  |  REAL MAP |  PLAYER MAP | TRAVEL LOG |
| NOTES  |  REAL MAP |  PLAYER MAP | TRAVEL LOG |
| NOTES  |  REAL MAP |  PLAYER MAP | TRAVEL LOG |
| NOTES  |  REAL MAP |  PLAYER MAP | TRAVEL LOG |
| NOTES  |  REAL MAP |  PLAYER MAP | TRAVEL LOG |
|-----------------------------------------------|
| Command line history                          |
|                                               |
|                                               |
|-----------------------------------------------|
| Command line                                  |
|-----------------------------------------------|


Maybe w/ the REST interface I could control Foundry _from_ this tool, which would be very cool indeed. If not, a websock into something custom could work.

ideally WFC happens here, for generating terrain automatically.

the ratatui maps will need a custom hex renderer, I think something like:


  ------
 /      \
/ A01234 \______
\SHRTMESG/      \
 \      /        \
  ------          -------
 /      \        /       \
/ A56789 \______/         \
\SHRTMESG/      \         /
 \      /        \       /
  ------          -------


A cell would be highlighted for party/player location on each screen, as well as actual location on each. Beneath the map might be a readout of the contents of the hex

Hexes can probably just be loaded/unloaded via Serde direct to files. a plane is just a folder of hexes, an unknown hex gets WFC'd.

Would be maybe cool to do a 'non-hex' system, where each hex is stored with adjaceny information, which would allow for a sort of natural non-euclidean,
non-uniform system that could contain both hex and depth crawls.

*/

