#![feature(adt_const_params)]

use truncheon::hex::{self, Direction, Origin};

fn main() {
    let field : hex::Hexfield<10, 10, {Origin::TopLeft}, {Direction::LR_TB}, ()> = hex::Hexfield::new();
    println!("Debug: {:?}", field);
}
