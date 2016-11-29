/// A playable level
pub trait LevelKind {
    /// The type of the position in the level
    type Position;

    /// Return the position of the player in the level
    fn get_position(&self) -> Self::Position;

    /// Returns whether the player fulfilled all goals of the level
    fn is_finished(&self) -> bool;
}
