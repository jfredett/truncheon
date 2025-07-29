#![allow(dead_code)]

use std::{collections::HashMap, marker::ConstParamTy};

#[derive(Debug, PartialEq)]
struct AxialCoord {
    q: isize,
    r: isize
}

struct CubicCoord {

}

/// Maps a coordinate to a index counting from the origin provided
struct IndexCoord<const ORIGIN: Origin, const DIRECTION: Direction> {

}

#[derive(ConstParamTy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Direction {
    LR_TB,
    RL_TB,
    LR_BT,
    RL_BT
}

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
}


pub enum HexMovement {
    N,
    NE,
    NW,
    S,
    SE,
    SW
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn it_works() {
        let field : Hexfield<10, 10, { Origin::TopLeft }, { Direction::LR_TB }, ()> = Hexfield::new();

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

