pub mod coord;
pub mod cardinal;
pub mod field;

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

// TODO:
// Something like a `World` struct which contains many `fields`, a `field` is a Hexfield, a
// Depthcrawl, a pointcrawl, etc. Fields contain `tiles` (`hex`es in the case of hexfield,
// locations for depth or pointcrawl), and `tiles` have some set of `connections`; for `hexes`
// there are 6 natural connections (to other hexes), but they might also connect to another field;
// or have one of their natural connections replaced with a connection to another field.
//
// On the `world` struct, there are some set of `cursor`s, which represent players, parties,
// enemies, or other mobile items.

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

