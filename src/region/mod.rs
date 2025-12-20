/// A topology, provides an answer to "Given a child of this region, what other children are it's neighbors?"
/// perhaps this should be a trait object?
#[derive(Component)]
pub enum Topology {
    Hex(HexCoordinateSystem),
}

pub enum HexCoordinateSystem {
    Axial,
    Cubic,
    Index,
    Offset,
}

#[derive(Component)]
pub struct Region;


#[derive(Component)]
pub struct Location;

impl Location {

}

pub struct Connection(Region, Location, Location);


// locations should be able to nest, so a location can contain arbitrary 'interior' locations,
// keeping the logic correct of 'smaller' inside 'bigger' is not enforced (TARDISes are possible)

impl Region {
    // maybe this should be a 'from-file' thing, so given some file in some format, spawn this and
    // all the locations associated therewith as relationships/children?
    pub fn spawn_empty_hex_region(mut commands: Commands, coord_system: HexCoordinateSystem) {
        let region = commands.spawn_empty()
            .insert(Region)
            .insert(Topology::Hex(coord_system))
    }

    pub fn load_hex_reagion(mut commands: Commands, file: ()) -> Result {

    }
}



            // .with_children(|parent| {
            //     /* snip -- locations go here, child is location + coordinate from the topology */
            //     /* Locations should be unique, and able to be in multiple regions at a time. 
            //     * so for instance, a location might be a child of `Isk`, but also of
            //     * `UpperCarpathia` for pointcrawling topology, and also be `YnnEntrance` for a
            //     * short time while a Ynn entrance is there, etc. Locations exist in multiple
            //     * regions with potentially different topologies. */
            // });
