use rustc_serialize::json;
use std::io::prelude::*;
use std::fs::File;

/// A single field of the world
#[derive(RustcEncodable, RustcDecodable, Clone)]
pub struct Field {
    /// The type of the field
    pub field_type: FieldType,
    /// The height of the field. Used for collision detection
    pub height: i32,
    /// The id if the contained entity (optional)
    pub contained_entity_id: Option<usize>,
}

impl Field {
    /// Creates a new instance of `Field`
    pub fn new(field_type: FieldType) -> Field {
        Field {
            field_type: field_type,
            height: 0,
            contained_entity_id: None,
        }
    }

    /// A builder method for adding an entity to a field
    pub fn contained_entity_id(mut self, entity_id: usize) -> Field {
        self.contained_entity_id = Some(entity_id);
        self
    }

    /// A builder method for setting the height of a field
    pub fn height(mut self, height: i32) -> Field {
        self.height = height;
        self
    }
}

/// The field type. Used to determine the optical properties of the ground
#[derive(RustcEncodable, RustcDecodable, Clone)]
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
#[derive(RustcEncodable, RustcDecodable)]
pub struct Level {
    /// The name or title of the level
    pub name: String,
    /// The entry point of the character
    pub starting_point: (usize, usize),
    /// The point where the level is finished
    pub end_point: (usize, usize),
    /// The actual size of the level
    size: (usize, usize),
    /// The actual fields, the level consists of
    data: Vec<Vec<Field>>,
}

impl Level {
    /// Creates a new instance of `Level`
    pub fn new(name: &str, size: (usize, usize)) -> Level {
        let (width, height) = size;
        Level {
            name: name.to_owned(),
            starting_point: (0, 0),
            end_point: (0, 0),
            size: (width, height),
            data: vec![vec![Field::new(FieldType::Grass); height]; width],
        }
    }

    /// A builder method for setting the starting point of the level
    pub fn starting_point(mut self, starting_point: (usize, usize)) -> Level {
        self.starting_point = starting_point;
        self
    }

    /// A builder method for setting the end point of the level
    pub fn end_point(mut self, end_point: (usize, usize)) -> Level {
        self.end_point = end_point;
        self
    }

    /// Sets the given field at the given position
    pub fn set_field(&mut self, field: Field, position: (usize, usize)) {
        let (x, y) = position;
        let (width, height) = self.size;
        assert!(x < width, "x is out of bounds");
        assert!(y < height, "y is out of bounds");
        self.data[x][y] = field;
    }
}

/// A collection of levels. Usually used to create larger adventures
#[derive(RustcEncodable, RustcDecodable)]
pub struct Campagne {
    /// The title of the campagne
    pub title: String,
    levels: Vec<Level>,
}

impl Campagne {
    /// Creates a new instance of `Campagne`
    pub fn new(title: &str) -> Campagne {
        Campagne {
            title: title.to_owned(),
            levels: Vec::new(),
        }
    }

    /// Adds a level to the campagne
    pub fn add_level(&mut self, level: Level) {
        self.levels.push(level);
    }

    /// Saves the campagne to the specified file
    pub fn save_to_file(&self, file_name: &str) {
        let mut f = match File::create(file_name) {
            Err(_) => return,
            Ok(file) => file,
        };

        let campagne = match json::encode(self) {
            Err(_) => return,
            Ok(campagne) => campagne,
        };

        match f.write_all(campagne.as_bytes()) {
            Err(_) => {}
            Ok(_) => {}
        };
    }

    /// Loads the campagen from the specified file
    pub fn load_from_file(file_name: &str) -> Result<Campagne, &str> {
        let mut f = match File::open(file_name) {
            Err(_) => return Err(file_name),
            Ok(file) => file,
        };

        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Err(_) => return Err(file_name),
            Ok(_) => {}
        };

        match json::decode(s.as_str()) {
            Err(_) => return Err(file_name),
            Ok(campagne) => Ok(campagne),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_and_load() {
        let camp = Campagne::new("Adventure Time!");
        camp.save_to_file("test.json");

        let new_camp = Campagne::load_from_file("test.json").ok().unwrap();

        assert_eq!(camp.title, new_camp.title);
    }

    #[test]
    fn build_campagne() {
        let mut camp = Campagne::new("Adventure Time!");

        let mut level = Level::new("Hunger Game", (10, 10));

        let field = Field::new(FieldType::Stone);

        level.set_field(field, (0, 0));

        camp.add_level(level);
    }

    #[test]
    fn new_level() {
        let mut level = Level::new("Hunger Game", (10, 10));

        level = level.starting_point((1, 2)).end_point((3, 4));

        assert_eq!(level.size.0, 10);
        assert_eq!(level.size.1, 10);
    }

    #[test]
    fn new_field() {
        let mut field = Field::new(FieldType::WoodenFence);

        field = field.contained_entity_id(23).height(2);
    }
}
