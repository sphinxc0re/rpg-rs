/// Structures for creating a linear world
pub mod linear;
/// Structures for creating a 2d world
pub mod two_dimensional;
/// A trait for implementing new world types
mod world_trait;
/// A container holding an arbitrary amount of worlds
pub mod campaign;

pub use self::world_trait::World;
