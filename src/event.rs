use item::Item;

#[derive(Clone)]
pub enum Event {
    Tell(String),
    Give(Item),
    Options(Vec<Event>),
    Push,
    Pull,
    Open,
    Close,
    Nothing,
}
