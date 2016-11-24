use entity::Entity;

pub struct Field {
    field_type: FieldType,
    walking_height: u8,
    contained_entity: Option<Entity>,
}

pub enum FieldType {
    None
}

pub struct Level {
    starting_point: (usize, usize),
    end_point: (usize, usize),
    data: Vec<Vec<Field>>,
}

pub struct Campagne {
    levels: Vec<Level>,
}
