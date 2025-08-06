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



use std::{collections::HashMap, ops::{Index, IndexMut}, sync::{Arc, Mutex, RwLock}};

use crate::hex::coord::axial;


#[derive(Debug, Default)]
pub struct Field<T> where T : Clone {
    // Lock the table to add new hexes, each hex is an arc-mutex. This means each T has it's own
    // lock. A hexmap can divide state across multiple fields for additional granularity.
    contents: RwLock<HashMap<axial::Point, Arc<Mutex<T>>>>
}

impl<T> Field<T> where T : Clone {
    pub fn new() -> Self {
        Field {
            contents: HashMap::new().into()
        }
    }

    pub fn insert(&self, key: axial::Point, value: T) {
        let mut contents = self.contents.write().unwrap();
        contents.insert(key, Arc::new(Mutex::new(value)));
    }

}

impl<T> Field<T> where T: Clone + Default {
    pub fn lookup(&self, key: axial::Point) -> Arc<Mutex<T>>{
        let mut contents = self.contents.write().unwrap();
        contents.entry(key).or_default().clone()
    }
}

// trait Fieldlike<T> {
//     fn lookup(&self, key: axial::Point) -> Arc<Mutex<T>>;
//     fn insert(&self, key: axial::Point, value: T);
// }



#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn insert_and_retrieve() {
        let mut field : Field<isize> = Field::new();
        let coord = axial::Point::new(0,0);
        let coord2 = axial::Point::new(0,1);
        let coord3 = axial::Point::new(1,1);

        field.insert(coord, 1);
        field.insert(coord2, 2);

        assert_eq!(field.lookup(coord), 1);
        assert_eq!(field.lookup(coord2), 2);

        assert_eq!(field.lookup(coord3), isize::default());
    }

}
