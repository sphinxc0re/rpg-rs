use item::Item;

/// A primitive event type
#[derive(Clone)]
pub enum Event {
    /// Talk to an entity
    Tell(String),
    /// Give an item to an entity
    Give(Item),
    /// Show a list events an entity can handle
    Options(Vec<Event>),
    /// Push an entity
    Push,
    /// Pull an entity
    Pull,
    /// Open an entity
    Open,
    /// Close an entity
    Close,
    /// Do nothing
    Nothing,
}
