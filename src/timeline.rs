// fundamentally, this is a log of events shared across other timelines.
//
// using something like fjall
//
// Universe             # Container for all other TLTs. Manages persistence.
// ID : u64             # stand in for any 8-byte UUID (or larger if it's easy)
// World                # Top Level Table
//  - Regions           # Reference Table, intended to be a container for, e.g., hexfields.
//      - Contnets      # Set of IDs of Localities
//  - Localities        # Reference Table (to either World or Region) -- ideally represents the smallest unit of 'place'-ness.
//      - Connections   # List of IDs of other Localities
//
// Timeline             # Top Level Table
//  - Subject           # Reference ID to the entity experiencing the timeline
//  - Log               # Ordered List of Event IDs
//
// Entity               # TLT -- maybe this is better thought of as 'an entity subject to time' or "EST", that abstracts over the two classes below
//  # Note, these may not be actual 'types', or they may be real types that implement the entity interface, idk.
//
//  - Pop               # A specific subtype of Entity representing something animate
//      - PC            # probably these aren't separate types
//      - NPC           # ibid
//  - Truc              # A subtype of entity represneting something _inanimate_ but with history, e.g., a kingdom, a named sword or magic item, etc.
//
// Event                # TLT
//  - Subject(s)        # Set of Entity IDs
//  - Location          # ID of the World
//  - Duration          # number of location-local seconds the event took.
//
// A top-level table (TLT) is a fjall-partition of the Universe Top-level keyspace.
//



// Rethink this a bit to fit with Bevy:
//
//
// struct Location;
// impl Location {
//   pub fn spawn_location(mut command: Command) {
//     command.spawn(
//        Location
//
//
//   }
//
// }

pub struct Universe {
    name: String,
    keyspace: Arc<RwLock<fjall::Keyspace>>
}

pub struct ID(u64);

pub struct World {
    universe: Universe, // writes get sent to the Universe async?
    id: ID,
    region: Vec<dyn Locality>
}


pub struct Place;

pub trait Locality {
    fn connections() -> Vec<dyn Locality>;
    fn contents() -> Vec<Place>;
}

pub trait Entity;

pub enum PopType {
    PC(owner: String),
    NPC
}

pub struct Pop {
    id: ID,
    type: PopType
}
impl Entity for Pop {}



pub struct Timeline {
    id: ID,
    name: String,
    sequence: Vec<Event>
}

pub struct Event {
    subjects: Vec<ID>, // the timelines that experience this event
    locality: ID, // where the event occurred
    duration: u64, // how many locality-local seconds occurred during this event
    id: ID
}




