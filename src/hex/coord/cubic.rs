
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

