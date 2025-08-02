/*
* I don't love this.
*
* I think the model I want is something like:
*
* Cursor -> A location on a map
*   Associated to one Map
*   Has name
*   can have vectors added, reset to point, etc.
*   has a log of all visited hexes
* Field<Hex> -> an independent axial coordinate system
*   Hashmap for now, eventually some flat indexing scheme would be nice.
* Hex -> An entry in a Map, this is abstract, and ideally two Maps should be "join"-able, i.e.:
*   Map<T>.join(Map<U>) -> Map<(T, U)>
*
* Cursors will have a small language a la `spell` from Hazel. The UI will hold a Field, which
* can contain references to other Fields in it, which the UI can then load. This might be bound up
* in a 'World' struct or something like.
*
* Fields are unbound, but centered on [0,0] by default. Display can have an 'offset' vector to
* control what it can see, and then I can calculate distance OTF. This drops the
* indexing/width-height reqs in the current model.
*
* This makes index-based storage harder, but I think the solution to that will be a spiral packing
* system. I can also use a straightforward conversion like `2^q * 3^r` or whatever to get a simple
* numeric value for each hex, even if it's sparse.
*
* 
*
*/



use std::{collections::HashMap, ops::{Index, IndexMut}};

use crate::hex::coord::axial;


#[derive(Debug)]
pub struct Field<T> where T : Clone {
    contents: HashMap<axial::Point, T>
}

impl<T> Field<T> where T : Clone {
    pub fn new() -> Self {
        Field {
            contents: HashMap::new()
        }
    }

    pub fn insert(&mut self, key: axial::Point, value: T) {
        self.contents.insert(key, value);
    }
}

impl<T, Idx> IndexMut<Idx> for Field<T> where T : Clone, Idx : Into<axial::Point> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        // FIXME: this is probably wrong.
        self.contents.get_mut(&index.into()).unwrap()
    }
}

impl<T, Idx> Index<Idx> for Field<T> where T : Clone, Idx : Into<axial::Point> {
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
        let mut field : Field<isize> = Field::new();
        let coord = axial::Point::new(0,0);
        let coord2 = axial::Point::new(0,1);

        field.insert(coord, 1);
        field.insert(coord2, 2);

        assert!(field[&coord] == 1);
        assert!(field[&coord2] != 1);
    }

}
