use entity::Entity;

/// A single field of the world
pub struct Field {
    /// The type of the field
    field_type: FieldType,
    /// The height of the field. Used for collision detection
    height: u8,
    /// The contained entity (optional)
    contained_entity: Option<Entity>,
}

/// The field type. Used to determine the optical properties of the ground
pub enum FieldType {
    /// A field consists of dirt
    Dirt,
    /// A field consists of grass
    Grass,
    /// A field a hole in the ground
    Hole,
    /// A field consists of mud
    Mud,
    /// A field consists of quicksand
    Quicksand,
    /// A field consists of sand
    Sand,
    /// A field consists of stone
    Stone,
    /// A field is a stone wall
    StoneWall,
    /// A field consists of swamp water
    SwampWater,
    /// A field consists of water
    Water,
    /// A field consists of wood
    Wood,
    /// A field is a wooded fence
    WoodenFence,
}

/// A larger section of a campagne containing a starting point and end point. The starting point
/// is where the character *spawns* and the end point is the point he has to reach for the next
/// level to begin.
pub struct Level {
    /// The name or title of the level
    name: String,
    /// The entry point of the character
    starting_point: (usize, usize),
    /// The point where the level is finished
    end_point: (usize, usize),
    /// The actual fields, the level consists of
    data: Vec<Vec<Field>>,
}

/// A collection of levels. Usually used to create larger adventures
pub struct Campagne {
    name: String,
    levels: Vec<Level>,
}
