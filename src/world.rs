//! The world, the story of the game is taking place in. A 2d grid consisting of simple fields
//! which usually consist of some material

use entity::Entity;

/// A single field of the world
pub struct Field {
    /// The type of the field
    field_type: FieldType,
    /// The height of the field. Used for collision detection
    height: u8,
    /// The contained entity, if present
    contained_entity: Option<Entity>,
}

/// The field type. Used to determine the properties of the ground or used for collision detection
pub enum FieldType {
    Dirt,
    Grass,
    Hole,
    Mud,
    Quicksand,
    Sand,
    Stone,
    StoneWall,
    SwampWater,
    Water,
    Wood,
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
