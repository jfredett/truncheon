pub mod point;
pub mod vector;

pub use point::*;
pub use vector::*;

// A CW spiral along a northward vector in a flat-top configuration
pub fn spiral() -> impl Iterator<Item = Point> {
    vec![
        Point::origin(),
        Point::new(0,-1),
        Point::new(1,-1),
        Point::new(1,0),
        Point::new(0,1),
        Point::new(-1,1),
        Point::new(-1,0),
    ].into_iter()
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn spiral_out__keep_going() {
        let mut spiral = spiral();
        assert_eq!(spiral.next(), Some(Point::origin()));
        assert_eq!(spiral.next(), Some(Point::new( 0,-1)));
        assert_eq!(spiral.next(), Some(Point::new( 1,-1)));
        assert_eq!(spiral.next(), Some(Point::new( 1, 0)));
        assert_eq!(spiral.next(), Some(Point::new( 0, 1)));
        assert_eq!(spiral.next(), Some(Point::new(-1, 1)));
        assert_eq!(spiral.next(), Some(Point::new(-1, 0)));
    }

}
