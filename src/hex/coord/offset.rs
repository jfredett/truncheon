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

