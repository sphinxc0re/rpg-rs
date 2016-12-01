/// A playable world
pub trait World {
    /// The type of the position in the world
    type Position;

    /// The type of the movement the player can make
    type Movement;

    /// Return the position of the player in the world
    fn get_position(&self) -> Self::Position;

    /// Returns whether the player fulfilled all goals of the world
    fn is_finished(&self) -> bool;

    /// Move the position marker to the specified position
    fn move_to(&mut self, movement: Self::Movement);
}
