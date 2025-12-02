# Commands

commands are of the form:

    COMMANDNAME PRIMARY_ARGUMENT? [KEY: VALUE]*

commands modify the state of the UI, which consists of all the things in the `state` section

Commands are the fundamental method of manipulation, and any shortcuts, hotkeys, mouse interactions, etc are defined in
terms of these commands (e.g., if I click on a party in a sidebar, it executes `select-party <NAME>` under the hood.

All commands (including these shortcutted ones) are tracked in the DM Audit Log. The Audit log can be used to replay
everything to any point.

Goal will be to import the details of rolls from Foundry into the log as well.

## State

1. Current Party: Option(Name)
    - this is the currently selected 'party', a party has arbitrary notes associated with it. Could be use for any
    collection of movable tokens on the map. They will have both their current real and presumed location (in the case
    of parties that can get lost).
    - Parties have travelogues, detailing where they went, both a GM and player version.
    - Arbitrary notes can attach to the travelogue.
2. Current Map
    - The currently displayed map.

## Party Management

- create-party NAME (with-token: PATH)?
    - Create a new party, optionally with a specific token, they will be placed in the 'Abyss', a magic off-map space
- select-party NAME
    - Select a party, either on this map or in the abyss
- place-party COORDS
    - place the currently selected party at the provided coordinates
- move DIRECTION
    - execute the move procedure on the party, which prompts for the result of any navigation rolls, etc, and calculates
      a 'get lost' direction.
- depth DEPTHCHART
    - Start a depthcrawl for the currently selected party, this is used for exploring certain regions, going into caves
      or dungeons, etc.
- note
    - opens an editor which allows editing the travelogue.

## Depth Crawling

- deeper
- back
- blindly

these should track a depthlogue similar to the travelogue, it uses some contextual set of tables to generate and track
the depthcrawl.

## Map Management

- load PATH
    - load the map and any saved parties attached to it.
- cursor COORDS
    - select the given hex cell
- seed COORDS
    - Edit the notes for the given hex cell

## Economy Tracking

- grant PARTYNAME (item: NAME) (xp: AMOUNT) (gold: AMOUNT) (other: ARBITRARY)
    - grant some resource (XP, Gold, item, etc).
    - this tracks the resource individually, has optional options for subtype, and if run without argument runs through
      a wizard/prompt thing.
- tax PARTYNAME <ibid>
    - same as grant, but in reverse.

NOTE: These may need to drill down to the character level?


----

Hexes should have ~6 'areas' within the hex each roughly a 1/6th of the hex, which can contain one of a number of
features, including structures, strange flora, strange fauna, unique terrain, encounters, or empty space. When entering
a hex, it should generate some of these from tables.

WFC should create otherwise unexplored hexes

Hexes have terrain information, at first just a terrain type, but eventually parameters what determine the type (e.g.,
rainfall, soil type, etc).
