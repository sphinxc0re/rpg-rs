use entity::Entity;
use super::World;

/// A single field of the world
#[derive(Clone)]
pub struct Field {
    /// The type of the field
    pub field_type: FieldType,
    /// The height of the field. Used for collision detection
    pub height: i32,
    /// The id if the contained entity (optional)
    pub entity: Option<Entity>,
}

impl Field {
    /// Creates a new instance of `Field`
    pub fn new(field_type: FieldType) -> Field {
        Field {
            field_type: field_type,
            height: 0,
            entity: None,
        }
    }

    /// A builder method for adding an entity to a field
    pub fn entity(mut self, entity: Entity) -> Field {
        self.entity = Some(entity);
        self
    }

    /// A builder method for setting the height of a field
    pub fn height(mut self, height: i32) -> Field {
        self.height = height;
        self
    }
}

/// The field type. Used to determine the optical properties of the ground
#[derive(Clone)]
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

/// A larger section of a campaign containing a starting point and end point. The starting point
/// is where the character *spawns* and the end point is the point he has to reach for the next
/// world to begin.
pub struct World2d {
    /// The name or title of the world
    pub name: String,
    /// The entry point of the character
    pub starting_point: (usize, usize),
    /// The point where the world is finished
    pub end_point: (usize, usize),
    /// The current position of the player
    current_position: (usize, usize),
    /// The actual size of the world
    size: (usize, usize),
    /// The actual fields, the world consists of
    data: Vec<Vec<Field>>,
}

impl World2d {
    /// Creates a new instance of `World2d`
    pub fn new(name: &str, size: (usize, usize)) -> World2d {
        let (width, height) = size;
        World2d {
            name: name.to_owned(),
            starting_point: (0, 0),
            end_point: (0, 0),
            current_position: (0, 0),
            size: (width, height),
            data: vec![vec![Field::new(FieldType::Grass); height]; width],
        }
    }

    /// A builder method for setting the starting point of the world
    pub fn starting_point(mut self, starting_point: (usize, usize)) -> World2d {
        assert!(self.is_valid_coord(starting_point));
        self.starting_point = starting_point;
        self.current_position = starting_point;
        self
    }

    /// A builder method for setting the end point of the world
    pub fn end_point(mut self, end_point: (usize, usize)) -> World2d {
        assert!(self.is_valid_coord(end_point));
        self.end_point = end_point;
        self
    }

    /// Sets the given field at the given position
    pub fn set_field(&mut self, field: Field, position: (usize, usize)) {
        assert!(self.is_valid_coord(position));
        self.data[position.0][position.1] = field;
    }

    fn is_valid_coord(&mut self, coords: (usize, usize)) -> bool {
        let (x, y) = coords;
        let (width, height) = self.size;

        let mut result = true;

        if x > width {
            result = false;
            println!("x is out of bounds");
        }

        if y > height {
            result = false;
            println!("y is out of bounds");
        }

        result
    }
}

impl World for World2d {
    type Position = (usize, usize);

    type Movement = (i64, i64);

    fn get_position(&self) -> Self::Position {
        self.current_position
    }

    fn is_finished(&self) -> bool {
        self.current_position == self.end_point
    }

    fn move_to(&mut self, movement: Self::Movement) {
        let (mut pos_x, mut pos_y) = self.current_position;
        let (mov_x, mov_y) = movement;

        pos_x = ((pos_x as i64) + mov_x) as usize;
        pos_y = ((pos_y as i64) + mov_y) as usize;

        self.current_position = (pos_x, pos_y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use world::campaign::Campaign;
    use entity::Entity;

    #[test]
    fn build_campaign() {
        let mut camp = Campaign::new("Adventure Time!");

        let mut world = World2d::new("Hunger Game", (10, 10));

        let field = Field::new(FieldType::Stone);

        world.set_field(field, (0, 0));

        camp.add_world(world);
    }

    #[test]
    fn new_world() {
        let mut world = World2d::new("Hunger Game", (10, 10));

        world = world.starting_point((1, 2)).end_point((3, 4));

        assert_eq!(world.size.0, 10);
        assert_eq!(world.size.1, 10);
    }

    #[test]
    fn new_field() {
        let mut field = Field::new(FieldType::WoodenFence);

        let entity = Entity::new("Michael");

        field = field.entity(entity).height(2);
    }
}
