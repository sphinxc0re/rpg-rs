use super::World;

/// A collection of worlds. Usually used to create larger adventures
pub struct Campaign<T: World> {
    /// The title of the campaign
    pub title: String,
    worlds: Vec<T>,
}

impl<T: World> Campaign<T> {
    /// Creates a new instance of `Campaign`
    pub fn new(title: &str) -> Campaign<T> {
        Campaign {
            title: title.to_owned(),
            worlds: Vec::new(),
        }
    }

    /// Adds a world to the campaign
    pub fn add_world(&mut self, world: T) {
        self.worlds.push(world);
    }
}
